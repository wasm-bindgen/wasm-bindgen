use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::{env, fs, process};

use anyhow::{anyhow, Context, Error};
use rouille::{Request, Response, Server};

use super::{Cli, TestMode, Tests};

pub(crate) fn spawn(
    addr: &SocketAddr,
    headless: bool,
    module: &'static str,
    tmpdir: &Path,
    cli: Cli,
    tests: Tests,
    test_mode: TestMode,
    isolate_origin: bool,
    benchmark: PathBuf,
) -> Result<Server<impl Fn(&Request) -> Response + Send + Sync>, Error> {
    let mut js_to_execute = String::new();

    let cov_import = if test_mode.no_modules() {
        "let __wbgtest_cov_dump = wasm_bindgen.__wbgtest_cov_dump;\n\
         let __wbgtest_module_signature = wasm_bindgen.__wbgtest_module_signature;"
    } else {
        "__wbgtest_cov_dump,__wbgtest_module_signature,"
    };

    let cov_dump = r#"
        // Dump the coverage data collected during the tests
        const coverage = __wbgtest_cov_dump();

        if (coverage !== undefined) {
            await fetch("/__wasm_bindgen/coverage", {
                method: "POST",
                headers: {
                    "Module-Signature": __wbgtest_module_signature(),
                },
                body: coverage
            });
        }
    "#;

    let bench_import = if test_mode.no_modules() {
        "let __wbgbench_import = wasm_bindgen.__wbgbench_import;
        let __wbgbench_dump = wasm_bindgen.__wbgbench_dump;"
    } else {
        "__wbgbench_import,__wbgbench_dump,"
    };

    let import_bench = r#"
        // Import the benchmark data before benches
        const response = await fetch("/__wasm_bindgen/bench/fetch");
        if (response.ok) {
            const array = await response.arrayBuffer();
            __wbgbench_import(new Uint8Array(array));
        }
    "#;

    let dump_bench = r#"
        // Dump the benchmark data collected during the benches
        const benchmark_dump = __wbgbench_dump();

        if (benchmark_dump !== undefined) {
            await fetch("/__wasm_bindgen/bench/dump", {
                method: "POST",
                body: benchmark_dump
            });
        }
    "#;

    let wbg_import_script = if test_mode.no_modules() {
        format!(
            r#"
            let Context = wasm_bindgen.WasmBindgenTestContext;
            let __wbgtest_console_debug = wasm_bindgen.__wbgtest_console_debug;
            let __wbgtest_console_log = wasm_bindgen.__wbgtest_console_log;
            let __wbgtest_console_info = wasm_bindgen.__wbgtest_console_info;
            let __wbgtest_console_warn = wasm_bindgen.__wbgtest_console_warn;
            let __wbgtest_console_error = wasm_bindgen.__wbgtest_console_error;
            {cov_import}
            {bench_import}
            let init = wasm_bindgen;
            "#,
        )
    } else {
        format!(
            r#"
            import {{
                WasmBindgenTestContext as Context,
                __wbgtest_console_debug,
                __wbgtest_console_log,
                __wbgtest_console_info,
                __wbgtest_console_warn,
                __wbgtest_console_error,
                {cov_import}
                {bench_import}
                default as init,
            }} from './{module}';
            "#,
        )
    };

    let nocapture = cli.nocapture || cli.bench;
    let is_bench = cli.bench;
    let args = cli.get_args(&tests);

    if test_mode.is_worker() {
        let mut worker_script = if test_mode.no_modules() {
            format!(r#"importScripts("{module}.js");"#)
        } else {
            String::new()
        };

        worker_script.push_str(&wbg_import_script);

        match test_mode {
            TestMode::DedicatedWorker { .. } => worker_script.push_str("const port = self\n"),
            TestMode::SharedWorker { .. } => worker_script.push_str(
                r#"
                addEventListener('connect', (e) => {
                    const port = e.ports[0]
                "#,
            ),
            TestMode::ServiceWorker { .. } => worker_script.push_str(
                r#"
                addEventListener('install', (e) => skipWaiting());
                addEventListener('activate', (e) => e.waitUntil(clients.claim()));
                addEventListener('message', (e) => {
                    const port = e.ports[0]
                "#,
            ),
            _ => unreachable!(),
        }

        worker_script.push_str(&format!(
            r#"
            const nocapture = {nocapture};
            const wrap = method => {{
                const on_method = `on_console_${{method}}`;
                self.console[method] = function (...args) {{
                    if (nocapture) {{
                        self.__wbg_test_output_writeln(...args);
                    }}
                    if (self[on_method]) {{
                        self[on_method](args);
                    }}
                    port.postMessage(["__wbgtest_" + method, args]);
                }};
            }};

            self.__wbg_test_invoke = f => f();
            self.__wbg_test_output_writeln = function (...args) {{
                port.postMessage(["__wbgtest_output_append", args.map(String).join(' ') + "\n"]);
            }}

            wrap("debug");
            wrap("log");
            wrap("info");
            wrap("warn");
            wrap("error");

            async function run_in_worker(tests) {{
                const wasm = await init("./{module}_bg.wasm");
                const t = self;
                const cx = new Context({is_bench});

                self.on_console_debug = __wbgtest_console_debug;
                self.on_console_log = __wbgtest_console_log;
                self.on_console_info = __wbgtest_console_info;
                self.on_console_warn = __wbgtest_console_warn;
                self.on_console_error = __wbgtest_console_error;

                {args}

                if ({is_bench}) {{
                    {import_bench}
                }}

                await cx.run(tests.map(s => wasm[s]));
                {cov_dump}

                if ({is_bench}) {{
                    {dump_bench}
                }}
            }}

            port.onmessage = function(e) {{
                let tests = e.data;
                run_in_worker(tests);
            }}
            "#,
        ));

        if matches!(
            test_mode,
            TestMode::SharedWorker { .. } | TestMode::ServiceWorker { .. }
        ) {
            worker_script.push_str("})");
        }

        let name = if matches!(test_mode, TestMode::ServiceWorker { .. }) {
            "service.js"
        } else {
            "worker.js"
        };
        let worker_js_path = tmpdir.join(name);
        fs::write(worker_js_path, worker_script).context("failed to write JS file")?;

        js_to_execute.push_str(&format!(
            r#"
            // Now that we've gotten to the point where JS is executing, update our
            // status text as at this point we should be asynchronously fetching the
            // Wasm module.
            document.getElementById('output').textContent = "Loading Wasm module...";
            {}

            port.addEventListener("message", function(e) {{
                // Checking the whether the message is from wasm_bindgen_test
                if(
                    e.data &&
                    Array.isArray(e.data) &&
                    e.data[0] &&
                    typeof e.data[0] == "string" &&
                    e.data[0].slice(0,10)=="__wbgtest_"
                ) {{
                    const method = e.data[0].slice(10);
                    const args = e.data.slice(1);

                    if (
                        method == "log" || method == "error" ||
                        method == "warn" || method == "info" ||
                        method == "debug"
                    ) {{
                        // In non-headless mode, forward worker console output to the main
                        // page's console so it appears in DevTools.
                        if (!{headless}) {{
                            console[method].apply(console, args[0]);
                        }}
                    }} else if (method == "output_append") {{
                        const el = document.getElementById("output");
                        if (!el.dataset.appended) {{
                            el.textContent = "";
                            el.dataset.appended = "1";
                        }}
                        el.textContent += args[0];
                    }}
                }}
            }});

            async function main(test) {{
                port.postMessage(test)
            }}

            const tests = [];
            "#,
            {
                let module = if test_mode.no_modules() {
                    "classic"
                } else {
                    "module"
                };

                match test_mode {
                    TestMode::DedicatedWorker { .. } => {
                        format!(
                            r#"const port = new Worker('worker.js', {{type: '{module}'}});
                            port.onerror = function(e) {{
                                console.error('Worker error:', e.message, e.filename, e.lineno);
                                document.getElementById('output').textContent += '\nWorker error: ' + e.message;
                            }};
                            "#
                        )
                    }
                    TestMode::SharedWorker { .. } => {
                        format!(
                            r#"
                            const worker = new SharedWorker("worker.js?random=" + crypto.randomUUID(), {{type: "{module}"}});
                            worker.onerror = function(e) {{
                                console.error('Worker error:', e.message, e.filename, e.lineno);
                                document.getElementById('output').textContent += '\nWorker error: ' + e.message;
                            }};
                            const port = worker.port;
                            port.start();
                            "#
                        )
                    }
                    TestMode::ServiceWorker { .. } => {
                        format!(
                            r#"
                            const url = "service.js?random=" + crypto.randomUUID();
                            const registration = await navigator.serviceWorker.register(url, {{type: "{module}"}});
                            if (registration.installing) {{
                                registration.installing.onerror = function(e) {{
                                    console.error('ServiceWorker error:', e.message);
                                    document.getElementById('output').textContent += '\nServiceWorker error: ' + e.message;
                                }};
                            }}
                            await new Promise((resolve) => {{
                                navigator.serviceWorker.addEventListener('controllerchange', () => {{
                                    if (navigator.serviceWorker.controller.scriptURL != location.href + url) {{
                                        throw "`wasm-bindgen-test-runner` does not support running multiple service worker tests at the same time"
                                    }}
                                    resolve();
                                }});
                            }});
                            const channel = new MessageChannel();
                            navigator.serviceWorker.controller.postMessage(undefined, [channel.port2]);
                            const port = channel.port1;
                            port.start();
                            "#
                        )
                    }
                    _ => unreachable!(),
                }
            }
        ));
    } else {
        js_to_execute.push_str(&wbg_import_script);

        js_to_execute.push_str(&format!(
            r#"
            // Now that we've gotten to the point where JS is executing, update our
            // status text as at this point we should be asynchronously fetching the
            // Wasm module.
            document.getElementById('output').textContent = "Loading Wasm module...";

            async function main(test) {{
                const wasm = await init('./{module}_bg.wasm');

                const cx = new Context({is_bench});
                window.on_console_debug = __wbgtest_console_debug;
                window.on_console_log = __wbgtest_console_log;
                window.on_console_info = __wbgtest_console_info;
                window.on_console_warn = __wbgtest_console_warn;
                window.on_console_error = __wbgtest_console_error;

                {args}

                if ({is_bench}) {{
                    {import_bench}
                }}

                await cx.run(test.map(s => wasm[s]));
                {cov_dump}

                if ({is_bench}) {{
                    {dump_bench}
                }}
            }}

            const tests = [];
            "#,
        ));
    }
    for test in tests.tests {
        js_to_execute.push_str(&format!("tests.push('{}');\n", test.export));
    }
    js_to_execute.push_str("main(tests);\n");

    let js_path = tmpdir.join("run.js");
    fs::write(js_path, js_to_execute).context("failed to write JS file")?;

    // For now, always run forever on this port. We may update this later!
    let tmpdir = tmpdir.to_path_buf();
    let srv = Server::new(addr, move |request| {
        // The root path gets our canned `index.html`. The two templates here
        // differ slightly in the default routing of `console.log`, going to an
        // HTML element during headless testing so we can try to scrape its
        // output.
        if request.url() == "/" {
            let s = if headless {
                include_str!("index-headless.html")
            } else {
                include_str!("index.html")
            };
            let s = s.replace("// {NOCAPTURE}", &format!("const nocapture = {nocapture};"));
            let s = if !test_mode.is_worker() && test_mode.no_modules() {
                s.replace(
                    "<!-- {IMPORT_SCRIPTS} -->",
                    &format!("<script src='{module}.js'></script>\n<script src='run.js'></script>"),
                )
            } else {
                s.replace(
                    "<!-- {IMPORT_SCRIPTS} -->",
                    "<script src='run.js' type=module></script>",
                )
            };

            let mut response = Response::from_data("text/html", s);

            if isolate_origin {
                set_isolate_origin_headers(&mut response)
            }

            return response;
        } else if request.url() == "/__wasm_bindgen/coverage" {
            let module_signature = request
                .header("Module-Signature")
                .expect("sent coverage data without module signature")
                .parse()
                .expect("sent invalid module signature");

            return if let Err(e) = handle_coverage_dump(module_signature, request) {
                let s: &str = &format!("Failed to dump coverage: {e}");
                log::error!("{s}");
                let mut ret = Response::text(s);
                ret.status_code = 500;
                ret
            } else {
                Response::empty_204()
            };
        } else if request.url() == "/__wasm_bindgen/bench/fetch" {
            return handle_benchmark_fetch(&benchmark);
        } else if request.url() == "/__wasm_bindgen/bench/dump" {
            return if let Err(e) = handle_benchmark_dump(&benchmark, request) {
                let s: &str = &format!("Failed to save benchmark: {e}");
                log::error!("{s}");
                let mut ret = Response::text(s);
                ret.status_code = 500;
                ret
            } else {
                Response::empty_204()
            };
        }

        // Otherwise we need to find the asset here. It may either be in our
        // temporary directory (generated files) or in the main directory
        // (relative import paths to JS). Try to find both locations.
        let mut response = try_asset(request, &tmpdir);
        if !response.is_success() {
            response = try_asset(request, ".".as_ref());
        }
        // Make sure browsers don't cache anything (Chrome appeared to with this
        // header?)
        response.headers.retain(|(k, _)| k != "Cache-Control");
        if isolate_origin {
            set_isolate_origin_headers(&mut response)
        }
        response
    })
    .map_err(|e| anyhow!("{e}"))?;
    return Ok(srv);

    fn try_asset(request: &Request, dir: &Path) -> Response {
        let response = rouille::match_assets(request, dir);
        if response.is_success() {
            return response;
        }

        // When a browser is doing ES imports it's using the directives we
        // write in the code that *don't* have file extensions (aka we say `from
        // 'foo'` instead of `from 'foo.js'`. Fixup those paths here to see if a
        // `js` file exists.
        if let Some(part) = request.url().split('/').next_back() {
            if !part.contains('.') {
                let new_request = Request::fake_http(
                    request.method(),
                    format!("{}.js", request.url()),
                    request
                        .headers()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                    Vec::new(),
                );
                let response = rouille::match_assets(&new_request, dir);
                if response.is_success() {
                    return response;
                }
            }
        }
        response
    }
}

fn handle_benchmark_fetch(path: &Path) -> Response {
    if let Ok(data) = std::fs::read(path) {
        Response::from_data("application/octet-stream", data)
    } else {
        Response::empty_400()
    }
}

fn handle_benchmark_dump(path: &Path, request: &Request) -> anyhow::Result<()> {
    let mut data = Vec::new();
    if let Some(mut body) = request.data() {
        body.read_to_end(&mut data)?;
    }
    std::fs::write(path, data)?;
    Ok(())
}

fn handle_coverage_dump(module_signature: u64, request: &Request) -> anyhow::Result<()> {
    // This is run after all tests are done and dumps the data received in the request
    // into a single profraw file
    let profraw_path = wasm_bindgen_test_shared::coverage_path(
        env::var("LLVM_PROFILE_FILE").ok().as_deref(),
        process::id(),
        env::temp_dir()
            .to_str()
            .context("failed to parse path to temporary directory")?,
        module_signature,
    );
    let mut profraw = std::fs::File::create(profraw_path)?;
    let mut data = Vec::new();
    if let Some(mut r_data) = request.data() {
        r_data.read_to_end(&mut data)?;
    }
    // Warnings about empty data should have already been handled by
    // the client

    profraw.write_all(&data)?;
    Ok(())
}

/*
 * Set the Cross-Origin-Opener-Policy and Cross-Origin_Embedder-Policy headers
 * on the Server response to enable worker context sharing, as described in:
 * https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy#certain_features_depend_on_cross-origin_isolation
 * https://security.googleblog.com/2018/07/mitigating-spectre-with-site-isolation.html
 */
fn set_isolate_origin_headers(response: &mut Response) {
    response.headers.push((
        Cow::Borrowed("Cross-Origin-Opener-Policy"),
        Cow::Borrowed("same-origin"),
    ));
    response.headers.push((
        Cow::Borrowed("Cross-Origin-Embedder-Policy"),
        Cow::Borrowed("require-corp"),
    ));
}

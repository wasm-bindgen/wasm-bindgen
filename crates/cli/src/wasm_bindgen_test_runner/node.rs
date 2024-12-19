use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use std::{env, fs};

use anyhow::bail;
use anyhow::{Context, Error};

use super::Cli;
use super::Tests;

// depends on the variable 'wasm' and initializes te WasmBindgenTestContext cx
pub fn shared_setup(is_bench: bool) -> String {
    format!(
        r#"
const handlers = {{}};

const wrap = method => {{
    const og = console[method];
    const on_method = `on_console_${{method}}`;
    console[method] = function (...args) {{
        if (nocapture) {{
            og.apply(this, args);
        }}
        if (handlers[on_method]) {{
            handlers[on_method](args);
        }}
    }};
}};

// save original `console.log`
global.__wbgtest_og_console_log = console.log;
// override `console.log` and `console.error` etc... before we import tests to
// ensure they're bound correctly in wasm. This'll allow us to intercept
// all these calls and capture the output of tests
wrap("debug");
wrap("log");
wrap("info");
wrap("warn");
wrap("error");

const cx = new wasm.WasmBindgenTestContext({is_bench});
handlers.on_console_debug = wasm.__wbgtest_console_debug;
handlers.on_console_log = wasm.__wbgtest_console_log;
handlers.on_console_info = wasm.__wbgtest_console_info;
handlers.on_console_warn = wasm.__wbgtest_console_warn;
handlers.on_console_error = wasm.__wbgtest_console_error;
"#
    )
}

pub fn execute(
    module: &str,
    tmpdir: &Path,
    cli: Cli,
    tests: Tests,
    module_format: bool,
    benchmark: PathBuf,
) -> Result<(), Error> {
    let coverage_env = if let Ok(env) = env::var("LLVM_PROFILE_FILE") {
        &format!("\"{env}\"")
    } else {
        "undefined"
    };
    let coverage_pid = process::id();
    let coverage_temp_dir = env::temp_dir()
        .to_str()
        .map(String::from)
        .context("failed to parse path to temporary directory")?;

    let mut js_to_execute = format!(
        r#"
        {exit};
        {fs};
        {wasm};

        const nocapture = {nocapture};
        {shared_setup}

        global.__wbg_test_invoke = f => f();

        async function main(tests) {{
            {args}

            if ({is_bench}) {{
                try {{
                    const benchmark_import = await fs.readFile('{benchmark}');
                    if (benchmark_import !== undefined)
                        wasm.__wbgbench_import(new Uint8Array(benchmark_import));
                }} catch {{
                }}
            }}

            const ok = await cx.run(tests.map(n => wasm.__wasm[n]));

            const coverage = wasm.__wbgtest_cov_dump();
            if (coverage !== undefined) {{
                const path = wasm.__wbgtest_coverage_path({coverage_env}, {coverage_pid}, {coverage_temp_dir:?}, wasm.__wbgtest_module_signature());
                await fs.writeFile(path, coverage);
            }}

            if ({is_bench}) {{
                const benchmark_dump = wasm.__wbgbench_dump();
                if (benchmark_dump !== undefined)
                    await fs.writeFile('{benchmark}', benchmark_dump);
            }}

            if (!ok)
                exit(1);
        }}

        const tests = [];
    "#,
        shared_setup = shared_setup(cli.bench),
        wasm = if !module_format {
            format!(r"const wasm = require('./{module}.js')")
        } else {
            format!(r"import * as wasm from './{module}.js'")
        },
        exit = if !module_format {
            r"const { exit } = require('node:process')".to_string()
        } else {
            r"import { exit } from 'node:process'".to_string()
        },
        fs = if !module_format {
            r"const fs = require('node:fs/promises')".to_string()
        } else {
            r"import fs from 'node:fs/promises'".to_string()
        },
        is_bench = cli.bench,
        nocapture = cli.nocapture || cli.bench,
        args = cli.get_args(&tests),
        benchmark = benchmark.display()
    );

    // Note that we're collecting *JS objects* that represent the functions to
    // execute, and then those objects are passed into Wasm for it to execute
    // when it sees fit.
    for test in tests.tests {
        js_to_execute.push_str(&format!("tests.push('{}')\n", test.export));
    }
    // And as a final addendum, exit with a nonzero code if any tests fail.
    js_to_execute.push_str(
        "
        main(tests)
            .then(() => {
                exit(0);
            })
            .catch(e => {
                console.error(e);
                exit(1);
            });
    ",
    );

    let js_path = if module_format {
        // fixme: this is a hack to make node understand modules
        let package_json = tmpdir.join("package.json");
        fs::write(&package_json, r#"{"type": "module"}"#).unwrap();
        tmpdir.join("run.mjs")
    } else {
        tmpdir.join("run.cjs")
    };
    fs::write(&js_path, js_to_execute).context("failed to write JS file")?;

    // Augment `NODE_PATH` so things like `require("tests/my-custom.js")` work
    // and Rust code can import from custom JS shims. This is a bit of a hack
    // and should probably be removed at some point.
    let path = env::var("NODE_PATH").unwrap_or_default();
    let mut path = env::split_paths(&path).collect::<Vec<_>>();
    path.push(env::current_dir().unwrap());
    path.push(tmpdir.to_path_buf());
    let extra_node_args = env::var("NODE_ARGS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let status = Command::new("node")
        .env("NODE_PATH", env::join_paths(&path).unwrap())
        .arg("--expose-gc")
        .args(&extra_node_args)
        .arg(&js_path)
        .status()
        .context("failed to find or execute Node.js")?;

    if !status.success() {
        bail!("Node failed with exit_code {}", status.code().unwrap_or(1))
    }

    Ok(())
}

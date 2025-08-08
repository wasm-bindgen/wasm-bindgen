use std::str;
use tokio::process::Command;

use example_tests::{example_dir, run, test_example};
use tokio::sync::OnceCell;

const PNPM: &str = if cfg!(windows) { "pnpm.cmd" } else { "pnpm" };

async fn test_webpack_example(name: &str) -> anyhow::Result<()> {
    test_example(name, || async {
        let path = example_dir(name);

        static INSTALL_DEPS: OnceCell<()> = OnceCell::const_new();

        INSTALL_DEPS
            .get_or_try_init(|| async {
                // Install deps (use PNPM to install entire workspace at once).
                run(Command::new(PNPM)
                    .arg("install")
                    .current_dir(example_dir(".")))
                .await
            })
            .await?;

        // Build the example.
        run(Command::new(PNPM).arg("build").current_dir(&path)).await?;

        Ok(path.join("dist"))
    })
    .await
}

#[allow(unused_macros)]
macro_rules! webpack_tests {
    ($(
        $(#[$attr:meta])*
        $test:ident = $name:literal,
    )*) => {
        $(
            $(#[$attr])*
            #[tokio::test]
            async fn $test() -> anyhow::Result<()> {
                test_webpack_example($name).await
            }
        )*
    };
}

#[cfg(feature = "stable")]
webpack_tests! {
    add = "add",
    canvas = "canvas",
    char = "char",
    closures = "closures",
    console_log = "console_log",
    dom = "dom",
    duck_typed_interfaces = "duck-typed-interfaces",
    fetch = "fetch",
    guide_supported_types_examples = "guide-supported-types-examples",
    hello_world = "hello_world",
    import_js = "import_js",
    julia_set = "julia_set",
    paint = "paint",
    performance = "performance",
    request_animation_frame = "request-animation-frame",
    todomvc = "todomvc",
    wasm_in_wasm_imports = "wasm-in-wasm-imports",
    wasm_in_wasm = "wasm-in-wasm",
    weather_report = "weather_report",
    webaudio = "webaudio",
    #[ignore = "The CI virtual machines don't have GPUs, so this doesn't work there."]
    webgl = "webgl",
    webrtc_datachannel = "webrtc_datachannel",
    #[ignore = "WebXR isn't supported in Firefox yet"]
    webxr = "webxr",
}

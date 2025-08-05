use tokio::process::Command;
use std::str;

use example_tests::{example_dir, run, test_example};

async fn test_shell_example(name: &str, envs: &[(&str, &str)]) -> anyhow::Result<()> {
    test_example(name, || async {
        let path = example_dir(name);
        run(Command::new(path.join("build.sh"))
            .current_dir(&path)
            .envs(envs.iter().copied())).await?;
        Ok(path)
    })
    .await
}

macro_rules! shell_tests {
    ($(
        $(#[$attr:meta])*
        $(#[$var:literal = $val:literal])*
        $test:ident = $name:literal,
    )*) => {
        $(
            #[cfg_attr(not(unix), ignore = "shell scripts are not supported on non-Unix OSes")]
            $(#[$attr])*
            #[tokio::test]
            async fn $test() -> anyhow::Result<()> {
                test_shell_example($name, &[$(($var, $val)),*]).await
            }
        )*
    };
}

shell_tests! {
    #[cfg(feature = "nightly")]
    #["RUSTUP_TOOLCHAIN" = "nightly"]
    raytrace_parallel = "raytrace-parallel",
    #[cfg(feature = "stable")]
    synchronous_instantiation = "synchronous-instantiation",
    #[cfg(feature = "nightly")]
    #["RUSTUP_TOOLCHAIN" = "nightly"]
    wasm_audio_worklet = "wasm-audio-worklet",
    #[cfg(feature = "stable")]
    wasm_in_web_worker = "wasm-in-web-worker",
    #[cfg(feature = "stable")]
    websockets = "websockets",
    #[cfg(feature = "stable")]
    without_a_bundler = "without-a-bundler",
    #[cfg(feature = "stable")]
    without_a_bundler_no_modules = "without-a-bundler-no-modules",
}

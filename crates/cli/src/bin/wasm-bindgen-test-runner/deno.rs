use std::path::Path;
use std::process::Command;
use std::{fs, process};

use anyhow::{Context, Error};

use crate::Tests;
use crate::{node::SHARED_SETUP, Cli};

pub fn execute(module: &str, tmpdir: &Path, cli: Cli, tests: Tests) -> Result<(), Error> {
    let mut js_to_execute = format!(
        r#"import * as wasm from "./{module}.js";

        const nocapture = {nocapture};
        {console_override}

        window.__wbg_test_invoke = f => f();

        {args}

        const tests = [];
    "#,
        nocapture = cli.nocapture.clone(),
        console_override = SHARED_SETUP,
        args = cli.into_args(&tests),
    );

    for test in tests.tests {
        js_to_execute.push_str(&format!("tests.push('{}')\n", test.export));
    }

    js_to_execute.push_str(
        r#"const ok = await cx.run(tests.map(n => wasm.__wasm[n]));
if (!ok) Deno.exit(1);"#,
    );

    let js_path = tmpdir.join("run.js");
    fs::write(&js_path, js_to_execute).context("failed to write JS file")?;

    /*
    // Augment `NODE_PATH` so things like `require("tests/my-custom.js")` work
    // and Rust code can import from custom JS shims. This is a bit of a hack
    // and should probably be removed at some point.
    let path = env::var("NODE_PATH").unwrap_or_default();
    let mut path = env::split_paths(&path).collect::<Vec<_>>();
    path.push(env::current_dir().unwrap());
    path.push(tmpdir.to_path_buf());
    let extra_node_args = env::var("NODE_ARGS")
        .unwrap_or_default()
        .split(",")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    exec(
        Command::new("node")
            .env("NODE_PATH", env::join_paths(&path).unwrap())
            .args(&extra_node_args)
            .arg(&js_path)
            .args(args),
    )*/
    let status = Command::new("deno")
        .arg("run")
        .arg("--allow-read")
        .arg(&js_path)
        .status()?;

    if !status.success() {
        process::exit(status.code().unwrap_or(1))
    } else {
        Ok(())
    }
}

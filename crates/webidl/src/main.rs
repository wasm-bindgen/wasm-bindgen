mod update_cargo_toml;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use update_cargo_toml::update_cargo_toml_features;

#[derive(Parser, Debug)]
#[clap(about = "Converts WebIDL into wasm-bindgen compatible code.")]
struct Opt {
    input_dir: PathBuf,

    output_dir: PathBuf,

    #[clap(long)]
    no_features: bool,

    #[clap(long)]
    next_unstable: bool,

    cargo_toml_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::parse();

    let features = !opt.no_features;
    let next_unstable = opt.next_unstable;

    let generated_features = wasm_bindgen_webidl::generate(
        &opt.input_dir,
        &opt.output_dir,
        wasm_bindgen_webidl::Options {
            features,
            next_unstable: std::cell::Cell::new(next_unstable),
        },
    )?;

    if let Some(cargo_toml_path) = opt.cargo_toml_path {
        if features {
            update_cargo_toml_features(&cargo_toml_path, &generated_features)?;
        } else {
            log::warn!("with no_features, not updating Cargo.toml");
        }
    }

    Ok(())
}

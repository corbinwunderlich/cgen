use std::{env, fs, path::PathBuf};

use clap::CommandFactory;

#[path = "src/cli.rs"]
mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("target")
        .join("man");

    fs::create_dir_all(&out_dir)?;

    let cmd = cli::Args::command();

    clap_mangen::generate_to(cmd, &out_dir)?;

    Ok(())
}

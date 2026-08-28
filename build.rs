use std::{env, fs, path::PathBuf};

use clap::CommandFactory;
use clap_complete::Shell;

#[path = "src/cli.rs"]
mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("target");

    let man_dir = out_dir.join("man");
    let completions_dir = out_dir.join("completions");

    fs::create_dir_all(&man_dir)?;
    fs::create_dir_all(&completions_dir)?;

    clap_mangen::generate_to(cli::Args::command(), &man_dir)?;

    macro_rules! generate_shell_completion {
        ($shell:expr) => {
            clap_complete::generate_to(
                $shell,
                &mut cli::Args::command(),
                "cgen",
                completions_dir.clone(),
            )?;
        };
    }

    generate_shell_completion!(Shell::Bash);
    generate_shell_completion!(Shell::Fish);
    generate_shell_completion!(Shell::Zsh);
    generate_shell_completion!(clap_complete_nushell::Nushell);

    Ok(())
}

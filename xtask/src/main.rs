use std::{fs, path::PathBuf, process};

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use schemars::schema_for;

#[derive(Parser)]
#[command(about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Manpages,
    Completions,
    JsonSchema,
}

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

const APP: &str = "cgen";

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{}", error);

        process::exit(-1);
    }
}

fn try_main() -> Result {
    let cli = Args::parse();

    match cli.command {
        Command::Manpages => manpages()?,
        Command::Completions => completions()?,
        Command::JsonSchema => json_schema()?,
    }

    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn manpages() -> Result {
    let out_dir = project_root().join("manpages");

    fs::create_dir_all(&out_dir)?;

    let mut buffer: Vec<u8> = Vec::new();

    let man = clap_mangen::Man::new(cgen::cli::Args::command());
    man.render(&mut buffer)?;

    fs::write(out_dir.join(format!("{}.1", APP)), buffer)?;

    Ok(())
}

fn completions() -> Result {
    let out_dir = project_root().join("completions");

    fs::create_dir_all(&out_dir)?;

    macro_rules! generate_shell_completion {
        ($shell:expr) => {
            clap_complete::generate_to($shell, &mut cgen::cli::Args::command(), APP, &out_dir)?;
        };
    }

    generate_shell_completion!(Shell::Bash);
    generate_shell_completion!(Shell::Fish);
    generate_shell_completion!(Shell::Zsh);
    generate_shell_completion!(Shell::PowerShell);
    generate_shell_completion!(Shell::Elvish);
    generate_shell_completion!(clap_complete_nushell::Nushell);

    Ok(())
}

fn json_schema() -> Result {
    let out_path = project_root().join("config.schema.json");

    let schema = schema_for!(cgen::cfg::Settings);

    fs::write(out_path, serde_json::to_string_pretty(&schema)?)?;

    Ok(())
}

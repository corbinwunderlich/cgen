use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::error;
use std::path::PathBuf;
use walkdir::WalkDir;

use clang::Clang;
use clap::Parser;

use crate::backends::{Backend, CHeader};

mod backends;
mod source;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Files or directories to transform
    #[arg(required = true)]
    path: Vec<PathBuf>,
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,
}

fn main() {
    let args = Args::parse();

    colog::default_builder()
        .filter_level(args.verbosity.into())
        .init();

    let clang = Clang::new();

    if let Err(error) = clang {
        error!("failed to initialize Libclang: {}", error);

        return;
    }

    let clang = clang.unwrap();

    if let Err(error) = args
        .path
        .into_iter()
        .try_for_each(|path| process_file(&clang, path))
    {
        error!("{}", error);
    }
}

fn process_file(
    clang: &Clang,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if path.is_dir() {
        for file in WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .filter(|e| {
                e.extension()
                    .is_some_and(|f| matches!(f.to_str(), Some("c") | Some("cpp")))
            })
        {
            process_file(clang, file)?;
        }

        return Ok(());
    }

    let index = clang::Index::new(clang, false, false);

    let parser = index.parser(&path).parse();

    if let Err(error) = parser {
        return Err(format!(
            "failed to parse file {}: {}",
            path.to_str().unwrap_or(""),
            error
        )
        .into());
    }

    let parser = parser.unwrap();

    let ranges = source::ranges_from_ast(&parser).ok_or(format!(
        "failed to get source ranges from file {}",
        path.to_str().unwrap_or("")
    ))?;

    let header = CHeader::new(&path);

    header.write(header.generate_content(ranges))?;

    Ok(())
}

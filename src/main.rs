use std::path::PathBuf;

use clang::Clang;
use clap::Parser;

use crate::backends::{Backend, CHeader};

mod backends;
mod source;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    path: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let clang = Clang::new();

    if let Err(error) = clang {
        eprintln!("Error: failed to initialize Libclang: {}", error);

        return;
    }

    let clang = clang.unwrap();

    if let Err(error) = args
        .path
        .into_iter()
        .try_for_each(|path| process_file(&clang, path))
    {
        eprintln!("Error: {}", error);
    }
}

fn process_file(
    clang: &Clang,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

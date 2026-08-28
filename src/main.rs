use clang::Clang;
use clap::Parser;
use miette::{Diagnostic, Report};
use std::{path::PathBuf, time};
use thiserror::Error;
use walkdir::WalkDir;

use crate::backends::{Backend, CHeader};

mod backends;
mod cli;
mod source;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to generate code from source {path}")]
#[diagnostic(forward(source))]
struct CgenError {
    pub path: PathBuf,
    #[source]
    pub source: CgenErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
enum CgenErrorKind {
    #[error("Failed to parse content")]
    Parse(#[from] clang::SourceError),
    #[error("Failed to get source ranges")]
    SourceRange,
    #[error(transparent)]
    #[diagnostic(transparent)]
    WriteError(#[from] crate::backends::WriteError),
}

fn main() -> Result<(), Report> {
    let args = crate::cli::Args::parse();

    let start = time::Instant::now();

    let clang = Clang::new().unwrap();

    let mut files_processed = 0u32;

    if let Err(error) = args.path.into_iter().try_for_each(|path| {
        match process_file(&mut files_processed, &clang, &path) {
            Err(error) => Err(CgenError {
                path,
                source: error,
            }),
            Ok(()) => Ok(()),
        }
    }) {
        return Err(error.into());
    }

    println!(
        "Generated {} files in {}ms",
        files_processed,
        start.elapsed().as_millis()
    );

    Ok(())
}

fn process_file(
    files_processed: &mut u32,
    clang: &Clang,
    path: &PathBuf,
) -> Result<(), CgenErrorKind> {
    if path.is_dir() {
        for file in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .filter(|e| {
                e.extension()
                    .is_some_and(|f| matches!(f.to_str(), Some("c") | Some("cpp")))
            })
        {
            process_file(files_processed, clang, &file)?;
        }

        return Ok(());
    }

    let index = clang::Index::new(clang, false, false);

    let parser = index.parser(path).parse()?;

    let ranges = source::ranges_from_ast(&parser).ok_or(CgenErrorKind::SourceRange)?;

    let header = CHeader::new(path);

    header.write(header.generate_content(ranges))?;

    *files_processed += 1;

    Ok(())
}

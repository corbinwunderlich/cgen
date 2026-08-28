use clang::Clang;
use clap::Parser;
use log::error;
use std::path::PathBuf;
use thiserror::Error;
use walkdir::WalkDir;

use crate::backends::{Backend, CHeader};

mod backends;
mod cli;
mod source;

#[derive(Debug, Error)]
#[error("while processing {path}:\n  {kind}")]
struct CgenError {
    pub path: PathBuf,
    pub kind: CgenErrorKind,
}

#[derive(Debug, Error)]
enum CgenErrorKind {
    #[error("failed to parse content")]
    Parse(#[from] clang::SourceError),
    #[error("failed to get source ranges")]
    SourceRange,
    #[error(transparent)]
    WriteError(#[from] crate::backends::WriteError),
}

fn main() {
    let args = crate::cli::Args::parse();

    colog::default_builder()
        .filter_level(args.verbosity.into())
        .init();

    let clang = Clang::new().unwrap();

    if let Err(error) =
        args.path
            .into_iter()
            .try_for_each(|path| match process_file(&clang, &path) {
                Err(error) => Err(CgenError { path, kind: error }),
                Ok(()) => Ok(()),
            })
    {
        error!("{}", error);
    }
}

fn process_file(clang: &Clang, path: &PathBuf) -> Result<(), CgenErrorKind> {
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
            process_file(clang, &file)?;
        }

        return Ok(());
    }

    let index = clang::Index::new(clang, false, false);

    let parser = index.parser(path).parse()?;

    let ranges = source::ranges_from_ast(&parser).ok_or(CgenErrorKind::SourceRange)?;

    let header = CHeader::new(path);

    header.write(header.generate_content(ranges))?;

    Ok(())
}

use clang::Clang;
use miette::{Diagnostic, Report};
use std::{ffi::OsStr, path::PathBuf, time};
use thiserror::Error;
use walkdir::WalkDir;

use crate::backends::{Backend, CHeader};

mod backends;
pub mod cfg;
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
    #[error("File extension '{extension}' is not allowed")]
    #[diagnostic(
        code(cgen::main::disallowed_extension),
        help = "Add the extension to inputs.extensions in the config file."
    )]
    DisallowedExtension { extension: String },
    #[error("Failed to parse content")]
    Parse(#[from] clang::SourceError),
    #[error("Failed to get source ranges")]
    SourceRange,
    #[error(transparent)]
    #[diagnostic(transparent)]
    WriteError(#[from] crate::backends::WriteError),
}

pub fn main() -> Result<(), Report> {
    crate::cli::load();
    crate::cfg::load()?;

    let start = time::Instant::now();

    let clang = Clang::new().unwrap();

    let mut files_processed = 0u32;

    if let Err(error) = crate::cli::Args::global().path.iter().try_for_each(|path| {
        match process_file(&mut files_processed, &clang, path) {
            Err(error) => Err(CgenError {
                path: path.to_owned(),
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

fn is_allowed_extension(path: &PathBuf) -> bool {
    path.extension().is_some_and(|e| {
        e.to_str().is_some_and(|e| {
            crate::cfg::Settings::global()
                .inputs
                .extensions
                .contains(&e.to_owned())
        })
    })
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
            .filter(is_allowed_extension)
        {
            process_file(files_processed, clang, &file)?;
        }

        return Ok(());
    }

    if !is_allowed_extension(path) {
        return Err(CgenErrorKind::DisallowedExtension {
            extension: path
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap()
                .to_owned(),
        });
    }

    let index = clang::Index::new(clang, false, false);

    let parser = index.parser(path).parse()?;

    let ranges = source::ranges_from_ast(&parser).ok_or(CgenErrorKind::SourceRange)?;

    let header = CHeader::new(path);

    header.write(header.generate_content(ranges))?;

    *files_processed += 1;

    Ok(())
}

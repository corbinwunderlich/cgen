use miette::{Diagnostic, Report};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU32, Ordering},
    time,
};
use thiserror::Error;
use walkdir::WalkDir;

use crate::{backends::Backend, frontends::Frontend};

mod backends;
pub mod cfg;
mod cli;
mod frontends;
mod watch;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to generate code from source {path}")]
#[diagnostic(forward(source))]
struct CgenError {
    pub path: PathBuf,
    #[source]
    pub source: CgenErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
pub enum CgenErrorKind {
    #[error("File extension '{extension}' is not allowed")]
    #[diagnostic(
        code(cgen::main::disallowed_extension),
        help = "Add the extension to inputs.extensions in the config file."
    )]
    DisallowedExtension { extension: String },
    #[error("Failed to parse content")]
    Parse(#[from] clang::SourceError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    FrontendError(#[from] crate::frontends::Error),
    #[error(transparent)]
    #[diagnostic(transparent)]
    WriteError(#[from] crate::backends::WriteError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    WatchError(#[from] crate::watch::WatchError),
}

static FILES_PROCESSED: AtomicU32 = AtomicU32::new(0);

pub fn main() -> Result<(), Report> {
    crate::cli::load();
    crate::cfg::load()?;

    if crate::cli::Args::global().watch {
        crate::watch::begin(&crate::cli::Args::global().path)?;

        return Ok(());
    }

    let start = time::Instant::now();

    if let Err(error) =
        crate::cli::Args::global()
            .path
            .iter()
            .try_for_each(|path| match process_file(path) {
                Err(error) => Err(CgenError {
                    path: path.to_owned(),
                    source: error,
                }),
                Ok(()) => Ok(()),
            })
    {
        return Err(error.into());
    }

    println!(
        "Generated {} files in {}ms",
        FILES_PROCESSED.load(Ordering::Relaxed),
        start.elapsed().as_millis()
    );

    Ok(())
}

pub fn process_file(path: &Path) -> Result<(), CgenErrorKind> {
    if path.is_dir() {
        for file in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .filter(|p| crate::frontends::LibClang::is_allowed_extension(p))
        {
            process_file(&file)?;
        }

        return Ok(());
    }

    if !crate::frontends::LibClang::is_allowed_extension(path) {
        return Err(CgenErrorKind::DisallowedExtension {
            extension: path
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap()
                .to_owned(),
        });
    }

    let frontend = crate::frontends::LibClang::new(path);

    let header = crate::backends::CHeader::new(path);

    header.write(header.generate_content(frontend.generate_ranges()?))?;

    FILES_PROCESSED.store(
        FILES_PROCESSED.load(Ordering::Relaxed) + 1,
        Ordering::Relaxed,
    );

    Ok(())
}

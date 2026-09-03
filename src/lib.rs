use ignore::WalkBuilder;
use miette::{Diagnostic, Report};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU32, Ordering},
    time,
};
use thiserror::Error;

mod backends;
pub mod cfg;
pub mod cli;
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
        help = "Add the extension to any inputs.[input].extensions in the config file."
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

    if let Err(error) = crate::cli::Args::global()
        .path
        .iter()
        .flat_map(|path| {
            if !path.is_dir() {
                return vec![path.clone()];
            }

            WalkBuilder::new(path)
                .hidden(crate::cli::Args::global().hidden)
                .ignore(crate::cli::Args::global().ignore)
                .git_global(crate::cli::Args::global().ignore)
                .git_ignore(crate::cli::Args::global().ignore)
                .git_exclude(crate::cli::Args::global().ignore)
                .build()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_some_and(|e| e.is_file()))
                .map(ignore::DirEntry::into_path)
                .filter(|path| crate::frontends::is_any_allowed_extension(path))
                .collect()
        })
        .try_for_each(|path| {
            match process_file(
                &crate::frontends::create_frontend(&path).ok_or(CgenError {
                    path: path.clone(),
                    source: CgenErrorKind::DisallowedExtension {
                        extension: path
                            .extension()
                            .unwrap_or(OsStr::new(""))
                            .to_str()
                            .unwrap_or("")
                            .to_string(),
                    },
                })?,
            ) {
                Err(error) => Err(CgenError {
                    path,
                    source: error,
                }),
                Ok(()) => Ok(()),
            }
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

fn write_to_backend<Backend: crate::backends::Backend>(
    frontend: &impl crate::frontends::Frontend,
    path: &Path,
) -> Result<(), CgenErrorKind> {
    let backend = Backend::new(path);

    backend.write(backend.generate_content(frontend.generate_ranges()?))?;

    Ok(())
}

pub fn process_file<F: crate::frontends::Frontend>(frontend: &F) -> Result<(), CgenErrorKind> {
    let path = frontend.source_path();

    crate::backends::for_each_backend!(write_to_backend, frontend, path);

    FILES_PROCESSED.fetch_add(1, Ordering::Relaxed);

    Ok(())
}

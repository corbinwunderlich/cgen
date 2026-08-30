use std::{
    fs,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use thiserror::Error;

mod c_header;
pub use c_header::{CHeader, CHeaderConfig};

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to write to {path}")]
#[diagnostic(forward(source))]
pub struct WriteError {
    pub path: PathBuf,
    #[source]
    pub source: WriteErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
pub enum WriteErrorKind {
    #[error("Failed to unwrap generated file contents")]
    #[diagnostic(code(cgen::backends::unwrap_content_failed))]
    UnwrapContent,
    #[error(transparent)]
    #[diagnostic(code(cgen::backends::io_error), help("Check your file permissions."))]
    Io(#[from] std::io::Error),
}

pub trait Backend {
    fn new(source_path: &Path) -> Self;

    fn out_path(&self) -> &PathBuf;

    fn generate_content(&self, ranges: Vec<crate::frontends::SourceRange>) -> Option<String>;

    fn write(&self, content: impl Into<Option<String>>) -> Result<(), WriteError> {
        if let Err(error) = fs::write(
            self.out_path(),
            content.into().ok_or(WriteError {
                path: self.out_path().to_owned(),
                source: WriteErrorKind::UnwrapContent,
            })?,
        ) {
            return Err(WriteError {
                path: self.out_path().to_owned(),
                source: WriteErrorKind::Io(error),
            });
        }

        Ok(())
    }
}

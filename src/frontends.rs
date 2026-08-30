use std::path::{Path, PathBuf};

use miette::Diagnostic;
use thiserror::Error;

mod libclang;
pub use libclang::{ClangConfig, LibClang};

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to parse {path}")]
#[diagnostic(forward(source))]
pub struct Error {
    pub path: PathBuf,
    #[source]
    pub source: ErrorKind,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ErrorKind {
    #[error("Failed to parse content")]
    #[diagnostic(code(cgen::frontends::libclang_source_error))]
    Parse(#[from] clang::SourceError),
    #[error("Failed to get source ranges")]
    #[diagnostic(code(cgen::frontends::source_range))]
    SourceRange,
}

#[derive(Clone)]
pub struct SourceRange {
    pub range: std::ops::Range<u32>,
    pub comment: Option<String>,
}

pub trait Frontend {
    fn is_allowed_extension(path: &Path) -> bool;

    fn generate_ranges(&self) -> Result<Vec<SourceRange>, Error>;
}

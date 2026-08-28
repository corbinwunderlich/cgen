mod c_header;

use std::{
    fs,
    path::{Path, PathBuf},
};

use log::info;

use thiserror::Error;

pub use c_header::CHeader;

#[derive(Debug, Error)]
#[error("while writing to {path}:\n    {kind}")]
pub struct WriteError {
    pub path: PathBuf,
    pub kind: WriteErrorKind,
}

#[derive(Debug, Error)]
pub enum WriteErrorKind {
    #[error("failed to unwrap generated file contents")]
    UnwrapContent,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub trait Backend {
    fn new(source_path: &Path) -> Self;

    fn source_path(&self) -> &PathBuf;
    fn out_path(&self) -> &PathBuf;

    fn generate_content(&self, ranges: Vec<crate::source::SourceRange>) -> Option<String>;

    fn write(&self, content: impl Into<Option<String>>) -> Result<(), WriteError> {
        if let Err(error) = fs::write(
            self.out_path(),
            content.into().ok_or(WriteError {
                path: self.out_path().to_owned(),
                kind: WriteErrorKind::UnwrapContent,
            })?,
        ) {
            return Err(WriteError {
                path: self.out_path().to_owned(),
                kind: WriteErrorKind::Io(error),
            });
        }

        info!(
            "wrote generated file {} from source file {}",
            self.out_path().to_str().unwrap_or(""),
            self.source_path().to_str().unwrap_or("")
        );

        Ok(())
    }
}

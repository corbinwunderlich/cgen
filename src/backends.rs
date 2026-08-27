mod c_header;

use std::{fs, path::PathBuf};

pub use c_header::CHeader;

pub trait Backend {
    fn new(source_path: &PathBuf) -> Self;

    fn source_path(&self) -> &PathBuf;
    fn out_path(&self) -> &PathBuf;

    fn generate_content(&self, ranges: Vec<crate::source::SourceRange>) -> Option<String>;

    fn write(&self, content: impl Into<Option<String>>) -> Result<(), String> {
        if let Err(error) = fs::write(
            self.out_path(),
            content.into().ok_or(format!(
                "Error: failed to generate file contents for source file {}",
                self.source_path().to_str().unwrap_or("")
            ))?,
        ) {
            return Err(format!(
                "Error: failed to write generated file for source file {}, {}",
                self.source_path().to_str().unwrap_or(""),
                error
            ));
        }

        Ok(())
    }
}

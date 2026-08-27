mod c_header;

use std::{
    fs,
    path::{Path, PathBuf},
};

pub use c_header::CHeader;

pub trait Backend {
    fn new(source_path: &Path) -> Self;

    fn source_path(&self) -> &PathBuf;
    fn out_path(&self) -> &PathBuf;

    fn generate_content(&self, ranges: Vec<crate::source::SourceRange>) -> Option<String>;

    fn write(&self, content: impl Into<Option<String>>) -> Result<(), String> {
        if let Err(error) = fs::write(
            self.out_path(),
            content.into().ok_or(format!(
                "failed to generate file contents for source file {}",
                self.source_path().to_str().unwrap_or("")
            ))?,
        ) {
            return Err(format!(
                "failed to write generated file for source file {}, {}",
                self.source_path().to_str().unwrap_or(""),
                error
            ));
        }

        println!(
            "Wrote generated file {} from source file {}",
            self.out_path().to_str().unwrap_or(""),
            self.source_path().to_str().unwrap_or("")
        );

        Ok(())
    }
}

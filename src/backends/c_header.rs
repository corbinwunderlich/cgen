use std::{
    fs,
    path::{Path, PathBuf},
};

use indoc::formatdoc;
use schemars::JsonSchema;
use serde::Deserialize;
use smart_default::SmartDefault;
use twox_hash::XxHash3_128;

#[derive(Debug, Deserialize, JsonSchema, SmartDefault)]
#[serde(default)]
pub struct Config {
    #[default("h")]
    #[schemars(example = &"hpp", example = &"h")]
    /// The output extension for the header files
    extension: String,
}

pub struct CHeader {
    source_path: PathBuf,
    generated_path: PathBuf,
}

impl crate::backends::Backend for CHeader {
    fn new(source_path: &Path) -> Self {
        Self {
            source_path: source_path.to_owned(),
            generated_path: source_path
                .with_extension(&crate::cfg::Settings::global().outputs.c_header.extension),
        }
    }

    fn out_path(&self) -> &PathBuf {
        &self.generated_path
    }

    fn generate_content(&self, ranges: Vec<crate::frontends::SourceRange>) -> Option<String> {
        let file_content = fs::read_to_string(&self.source_path).ok()?;

        let result = ranges
            .into_iter()
            .fold(
                String::new(),
                |mut accumulator, crate::frontends::SourceRange { range, comment }| {
                    if let Some(comment) = comment {
                        accumulator.push_str(&(comment + "\n"));
                    }

                    let Some(source) = file_content.get(range.start as usize..range.end as usize)
                    else {
                        return accumulator;
                    };

                    accumulator.push_str(source);

                    Self::restore_semicolons(&mut accumulator);

                    accumulator
                },
            )
            .trim()
            .to_owned();

        Some(Self::add_header_boilerplate(result))
    }
}

impl CHeader {
    fn restore_semicolons(source: &mut String) {
        if let trimmed = source.trim_end()
            && !trimmed.ends_with(';')
        {
            *source = trimmed.to_owned() + ";";
        }

        source.push_str("\n\n");
    }

    fn add_header_boilerplate(content: String) -> String {
        let hash = XxHash3_128::oneshot(content.as_bytes());

        formatdoc! {"
            // clang-format off
            // NOLINTBEGIN

            #pragma once

            #ifndef __CGEN_{1:032X}_H
            #define __CGEN_{1:032X}_H

            {0}

            #endif /* __CGEN_{1:032X}_H */

            // clang-format on
            // NOLINTEND
        ", content, hash}
    }
}

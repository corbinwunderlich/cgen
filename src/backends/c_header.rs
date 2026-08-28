use std::{
    fs,
    path::{Path, PathBuf},
};

use clang::EntityKind;
use indoc::formatdoc;
use serde::Deserialize;
use smart_default::SmartDefault;
use twox_hash::XxHash3_128;

#[derive(Debug, Deserialize, SmartDefault)]
#[serde(default)]
pub struct Config {
    #[default("h")]
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

    fn generate_content(&self, ranges: Vec<crate::source::SourceRange>) -> Option<String> {
        let file_content = fs::read_to_string(&self.source_path).ok()?;

        let result = ranges
            .into_iter()
            .fold(String::new(), |mut accumulator, range| {
                if let Some(comment) = range.1.get_comment() {
                    accumulator.push_str(&(comment + "\n"));
                }

                let source = file_content.get(range.0.start as usize..range.0.end as usize);

                if source.is_none() {
                    return accumulator;
                }

                let mut source = source.unwrap().to_owned();

                if range.1.get_kind() == EntityKind::FunctionDecl {
                    Self::delete_function_body(&mut source);
                }

                accumulator.push_str(&source);

                Self::restore_semicolons(&mut accumulator);

                accumulator
            })
            .trim()
            .to_owned();

        Some(Self::add_header_boilerplate(result))
    }
}

impl CHeader {
    fn delete_function_body(source: &mut String) {
        let opening_brace = source.find('{');

        let closing_brace = source.rfind('}').map(|pos| {
            let remaining = &source[pos + 1..];

            let mut i = 0usize;
            for c in remaining.chars() {
                if !c.is_whitespace() {
                    break;
                }

                i += 1;
            }

            pos + i
        });

        if opening_brace.is_none() || closing_brace.is_none() {
            return;
        }

        source.replace_range(opening_brace.unwrap()..closing_brace.unwrap() + 1, "");
    }

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

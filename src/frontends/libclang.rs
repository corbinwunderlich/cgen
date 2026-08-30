use std::{
    cell::OnceCell,
    collections, ops,
    path::{Path, PathBuf},
};

use clang::{Clang, EntityKind};
use schemars::JsonSchema;
use serde::Deserialize;
use smart_default::SmartDefault;

use super::SourceRange;

#[derive(Debug, Deserialize, JsonSchema, SmartDefault)]
#[serde(default)]
pub struct ClangConfig {
    #[default(["c", "cpp", "cc", "cxx", "c++", "m", "mm"].iter().map(|e| e.to_string()).collect())]
    /// The extensions which are parsed by Clang
    pub extensions: Vec<String>,
}

struct GlobalClang(OnceCell<Clang>);
unsafe impl Sync for GlobalClang {}

static CLANG: GlobalClang = GlobalClang(OnceCell::new());

impl GlobalClang {
    fn get() -> &'static Clang {
        CLANG.0.get_or_init(|| clang::Clang::new().unwrap())
    }
}

fn find_function_body_range<'a>(
    node: &clang::Entity<'a>,
) -> Option<clang::source::SourceRange<'a>> {
    if node.get_kind() != EntityKind::FunctionDecl {
        return None;
    }

    if !node.is_definition() {
        return None;
    }

    let body = node
        .get_children()
        .into_iter()
        .find(|child| child.get_kind() == EntityKind::CompoundStmt)?;

    body.get_range()
}

fn delete_function_body(node: &clang::Entity<'_>, function_range: &mut ops::Range<u32>) {
    let body_range = find_function_body_range(node);

    if body_range.is_none() {
        return;
    }

    let body_range = body_range.unwrap();

    function_range.end = body_range.get_start().get_file_location().offset - 1;
}

fn find_overlapping_ranges<'r>(
    ranges: &'r [SourceRange],
    start: &u32,
    end: &u32,
) -> Vec<(usize, &'r SourceRange)> {
    ranges
        .iter()
        .enumerate()
        .filter(|(_, SourceRange { range, .. })| range.start < *end && *start < range.end)
        .collect()
}

fn contains_larger_range(ranges: &[(usize, &SourceRange)], start: &u32, end: &u32) -> bool {
    ranges.iter().any(|(_, SourceRange { range, .. })| {
        let range_length = end - start;

        range.len() < range_length as usize
    })
}

fn remove_overlapping_ranges(
    ranges: &[SourceRange],
    overlapping_ranges: Vec<(usize, &SourceRange)>,
) -> Vec<SourceRange> {
    let overlapping_indices: collections::HashSet<usize> = overlapping_ranges
        .into_iter()
        .map(|(index, _)| index)
        .collect();

    ranges
        .iter()
        .enumerate()
        .filter(|(index, _)| !overlapping_indices.contains(index))
        .map(|(_, range)| range)
        .cloned()
        .collect()
}

fn range_from_node(ranges: Vec<SourceRange>, node: clang::Entity<'_>) -> Option<Vec<SourceRange>> {
    let mut ranges = ranges;

    let range = node.get_range()?;

    let start = range.get_start().get_file_location();
    let end = range.get_end().get_file_location();

    if start.file.is_none() || end.file.is_none() {
        return None;
    }

    let overlapping_ranges = find_overlapping_ranges(&ranges, &start.offset, &end.offset);

    let mut range = start.offset..end.offset + 1;

    delete_function_body(&node, &mut range);

    if overlapping_ranges.is_empty() {
        ranges.push(SourceRange {
            range,
            comment: node.get_comment(),
        });

        return Some(ranges);
    }

    if contains_larger_range(&overlapping_ranges, &start.offset, &end.offset) {
        return Some(ranges);
    }

    ranges = remove_overlapping_ranges(&ranges, overlapping_ranges);

    ranges.push(SourceRange {
        range,
        comment: node.get_comment(),
    });

    Some(ranges)
}

pub struct LibClang {
    source_path: PathBuf,
}

impl crate::frontends::Frontend for LibClang {
    fn new(source_path: &Path) -> Self {
        LibClang {
            source_path: source_path.into(),
        }
    }

    fn source_path(&self) -> &Path {
        &self.source_path
    }

    fn is_allowed_extension(path: &Path) -> bool {
        path.extension().is_some_and(|e| {
            e.to_str().is_some_and(|e| {
                crate::cfg::Settings::global()
                    .inputs
                    .clang
                    .extensions
                    .contains(&e.into())
            })
        })
    }

    fn generate_ranges(&self) -> Result<Vec<SourceRange>, super::Error> {
        let index = clang::Index::new(GlobalClang::get(), false, false);

        let parser = index
            .parser(self.source_path.clone())
            .parse()
            .map_err(|e| super::Error {
                path: self.source_path.clone(),
                source: super::ErrorKind::Parse(e),
            })?;

        let ranges = parser
            .get_entity()
            .get_children()
            .into_iter()
            .filter(|node| node.get_linkage() != Some(clang::Linkage::Internal))
            .try_fold(Vec::new(), range_from_node)
            .ok_or(super::Error {
                path: self.source_path.clone(),
                source: super::ErrorKind::SourceRange,
            })?;

        Ok(ranges)
    }
}

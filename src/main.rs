use std::{collections, env, fs, path::{Path, PathBuf}};
use clang::{Clang, EntityKind};
use twox_hash::XxHash3_128;

fn main() {
    let mut args = env::args();

    args.next();

    if args.len() < 1 {
        eprintln!("Error: no path given");
        ()
    }

    let clang = Clang::new();

    if let Err(error) = clang {
        eprintln!("Error: failed to initialize Libclang: {}", error);

        return ();
    }

    let clang = clang.unwrap();

    if let Err(error) = args.try_for_each(|path| process_file(&clang, path)) {
        eprintln!("{}", error.to_string());
    }
}

fn get_adjacent_header_path(path: &str) -> PathBuf {
    const HEADER_EXTENSION: &str = "h";

    Path::new(path).with_extension(HEADER_EXTENSION)
}

fn write_header_file(path: &str, content: &str)-> Result<(), String> {
    if let Err(error) = fs::write(get_adjacent_header_path(path), content)  {
        return Err(format!("Error: failed to write header file for source file {}, {}", path, error));
    }

    Ok(())
}

#[derive(Clone)]
struct SourceRange<'a>(std::ops::Range<u32>, clang::Entity<'a>);

impl<'a> SourceRange<'a> {
    fn len(&self) -> u32 {
        self.0.end - self.0.start
    }
}

fn process_file(clang: &Clang, path: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let index = clang::Index::new(clang, false, false);

    let parser = index.parser(&path).parse();

    if let Err(error) = parser {
        return Err(format!("Error: failed to parse file {}: {}", path, error.to_string()).into())
    }

    let parser = parser.unwrap();

    let nodes: Vec<_> = parser
        .get_entity()
        .get_children()
        .into_iter()
        .filter(|node| node.get_linkage() != Some(clang::Linkage::Internal))
        .collect();

    let mut ranges: Vec<SourceRange> = Vec::new();

    for node in nodes {
        ranges = get_source_range_from_node(ranges, node)
            .ok_or(format!("Error: failed to get source range from node in file {}", path))?;
    }

    let header_content = process_header_content(get_source_from_ranges(&path, ranges).unwrap());

    write_header_file(&path, &header_content)?;

    Ok(())
}

fn get_source_range_from_node<'a>(mut ranges: Vec<SourceRange<'a>>, node: clang::Entity<'a>) -> Option<Vec<SourceRange<'a>>> {
    let range = node.get_range()?;

    let start = range.get_start().get_file_location();
    let end = range.get_end().get_file_location();

    if start.file.is_none() || end.file.is_none() {
        return None;
    }

    let overlapping_ranges: Vec<(usize, SourceRange)> = ranges
        .iter()
        .enumerate()
        .filter(|(_, SourceRange(r, _))| r.start < end.offset && start.offset < r.end)
        .map(|(i, r)| (i, r.clone()))
        .collect();

    let range = start.offset..end.offset + 1;

    if overlapping_ranges.len() == 0 {
        ranges.push(SourceRange(range, node));

        return Some(ranges);
    }

    if !overlapping_ranges.iter().any(|(_, r)| {
        let range_length = end.offset - start.offset;

        r.len() < range_length
    }) {
        return Some(ranges)
    }

    let overlapping_ranges: collections::HashSet<usize> = overlapping_ranges
        .into_iter()
        .map(|(index, _)| index)
        .collect();

    let mut i = 0usize;
    ranges.retain(|_| {
        let keep = !overlapping_ranges.contains(&i);

        i += 1;

        keep
    });

    ranges.push(SourceRange(range, node));

    Some(ranges)
}

fn get_source_from_ranges(source_path: &str, ranges: Vec<SourceRange>) -> Option<String> {
    let file_content = fs::read_to_string(source_path).ok()?;

    let result = ranges.into_iter().fold(String::new(), |mut accumulator, range| {
        match range.1.get_comment() {
            Some(comment) => accumulator.push_str(&(comment + "\n")),
            None => {}
        }

        let source = file_content.get(range.0.start as usize..range.0.end as usize);

        if let None = source {
            return accumulator;
        }

        let mut source = source.unwrap().to_owned();

        match range.1.get_kind() {
            EntityKind::FunctionDecl => 'arm: {
                let opening_brace = source.find('{');
                let closing_brace = source.rfind('}').and_then(|pos| {
                    let remaining = &source[pos + 1..];

                    let mut i = 0usize;
                    for c in remaining.chars() {
                        if !c.is_whitespace() {
                            break;
                        }

                        i += 1;
                    }

                    Some(pos + i)
                });

                if opening_brace.is_none() || closing_brace.is_none() {
                    break 'arm;
                }

                source.replace_range(opening_brace.unwrap()..closing_brace.unwrap() + 1, "");
            }
            _ => {}
        }

        accumulator.push_str(&source);

        if let trimmed = accumulator.trim_end() && trimmed.chars().last() != Some(';') {
            accumulator = trimmed.to_owned() + ";";
        }

        accumulator + "\n\n"
    }).trim().to_owned();

    Some(result)
}

fn process_header_content(content: String) -> String {
    let hash = XxHash3_128::oneshot(content.as_bytes());

    format!("
// clang-format off
// NOLINTBEGIN

#pragma once

#ifndef __CGEN_{1:032X}_H
#define __CGEN_{1:032X}_H

{0}

#endif /* __CGEN_{1:032X}_H */

// clang-format on
// NOLINTEND
    ", content, hash).trim_start().to_owned()
}

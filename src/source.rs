use std::collections;

#[derive(Clone)]
pub struct SourceRange<'a>(pub std::ops::Range<u32>, pub clang::Entity<'a>);

impl<'a> SourceRange<'a> {
    fn len(&self) -> u32 {
        self.0.end - self.0.start
    }
}

pub fn ranges_from_ast<'tu>(tu: &'tu clang::TranslationUnit<'tu>) -> Option<Vec<SourceRange<'tu>>> {
    tu.get_entity()
        .get_children()
        .into_iter()
        .filter(|node| node.get_linkage() != Some(clang::Linkage::Internal))
        .try_fold(Vec::new(), get_source_range_from_node)
}

fn get_source_range_from_node<'a>(
    mut ranges: Vec<SourceRange<'a>>,
    node: clang::Entity<'a>,
) -> Option<Vec<SourceRange<'a>>> {
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

    if overlapping_ranges.is_empty() {
        ranges.push(SourceRange(range, node));

        return Some(ranges);
    }

    if !overlapping_ranges.iter().any(|(_, r)| {
        let range_length = end.offset - start.offset;

        r.len() < range_length
    }) {
        return Some(ranges);
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

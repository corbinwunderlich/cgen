#![cfg(feature = "watch")]

use std::{ffi::OsStr, path::PathBuf};

use inotify::{Inotify, WatchDescriptor, WatchMask};
use miette::Diagnostic;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to watch file or directory")]
pub enum WatchError {
    #[error(transparent)]
    #[diagnostic(code(cgen::watch::io_error))]
    Io(#[from] std::io::Error),
}

struct Watcher<'a> {
    wd: WatchDescriptor,
    path: &'a PathBuf,
}

fn full_path_from_event(watchers: &[Watcher], event: inotify::Event<&OsStr>) -> PathBuf {
    let watcher = watchers
        .iter()
        .find(|Watcher { wd, .. }| *wd == event.wd)
        .unwrap();

    match event.name {
        Some(path) => watcher.path.join(path),
        None => watcher.path.clone(),
    }
}

pub fn begin(paths: &'static Vec<PathBuf>) -> Result<(), WatchError> {
    let mut inotify = Inotify::init()?;

    let watched_paths: Vec<PathBuf> = paths
        .iter()
        .flat_map(|path| {
            if !path.is_dir() {
                return vec![path.clone()];
            }

            WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_dir())
                .map(|d| d.into_path())
                .collect()
        })
        .collect();

    loop {
        let watchers: Vec<Watcher> = watched_paths.iter().try_fold(
            Vec::with_capacity(paths.len()),
            |mut accumulator, path| -> std::io::Result<Vec<Watcher>> {
                let wd = inotify.watches().add(
                    path,
                    WatchMask::CREATE | WatchMask::MODIFY | WatchMask::MOVED_TO,
                )?;

                accumulator.push(Watcher { wd, path });

                Ok(accumulator)
            },
        )?;

        let mut buffer = vec![0u8; 4096];

        let events = inotify.read_events_blocking(&mut buffer)?;

        for event in events {
            let path = full_path_from_event(&watchers, event);

            println!("Path {:#?} changed, generating...", path);

            crate::process_file(&path).unwrap();
        }
    }
}

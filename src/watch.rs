use std::{path::PathBuf, sync::mpsc, time::Duration};

use miette::Diagnostic;
use notify::Watcher;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to watch file or directory")]
pub enum WatchError {
    #[error(transparent)]
    #[diagnostic(code(cgen::watch::error))]
    Notify(#[from] notify::Error),
}

pub fn begin(paths: &'static Vec<PathBuf>) -> Result<(), WatchError> {
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher = notify::PollWatcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_millis(
            crate::cli::Args::global().watch_interval,
        )),
    )?;

    for path in paths {
        watcher.watch(path, notify::RecursiveMode::Recursive)?;
    }

    rx.iter()
        .filter_map(|event| event.ok())
        .flat_map(|event| event.paths)
        .filter_map(|path| {
            crate::frontends::create_frontend(&path).map(|frontend| (path, frontend))
        })
        .for_each(|(path, frontend)| {
            println!("Path {:#?} changed, generating...", path);

            crate::process_file(frontend).unwrap();
        });

    Ok(())
}

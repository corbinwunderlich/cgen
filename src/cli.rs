#![allow(dead_code)]

use std::{path::PathBuf, sync::OnceLock};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// Files or directories to transform
    #[arg(required = true)]
    pub path: Vec<PathBuf>,
    /// Path to config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Watch for changes in files/directories
    #[arg(short, long, default_value_t = false)]
    pub watch: bool,
    #[arg(long, default_value_t = 100)]
    /// Polling interval for --watch, in milliseconds
    pub watch_interval: u64,
    #[arg(short = 'I', long, default_value_t = false)]
    /// Respect .gitignore and .ignore when transforming directories (on by default)
    pub ignore: bool,
    #[arg(short = 'H', long, default_value_t = false)]
    /// Allow transforming hidden files and directories (off by default)
    pub hidden: bool,
}

static ARGS: OnceLock<Args> = OnceLock::new();

impl Args {
    pub fn global() -> &'static Self {
        ARGS.get().unwrap()
    }
}

pub fn load() {
    ARGS.set(Args::parse()).unwrap();
}

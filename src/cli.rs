#![allow(dead_code)]

use std::{path::PathBuf, sync::OnceLock};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// Files or directories to transform
    #[arg(required = true)]
    pub path: Vec<PathBuf>,
    /// Path to config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
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

use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Files or directories to transform
    #[arg(required = true)]
    pub path: Vec<PathBuf>,
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
}

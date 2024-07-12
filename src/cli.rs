use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about)]
pub(crate) struct Args {
    /// File to open
    pub file: Option<PathBuf>,
}

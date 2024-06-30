use std::path::PathBuf;

use clap::Parser;

/// wc - word, line, character, and byte count
#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short = 'c', help = "Output the bytes in a file")]
    pub bytes: bool,

    pub file_path: Option<PathBuf>,
}

pub fn get_args() -> Args {
    let args = Args::parse();

    args
}

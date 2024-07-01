use std::fs;
use std::path::PathBuf;

use clap::Parser;

pub fn build_output() -> Result<String, std::io::Error> {
    let Args {
        bytes,
        lines,
        file_path,
    } = get_args();

    let input = parse_input(&file_path)?;
    let mut output = String::new();

    if bytes == true {
        let byte_count = input.len();
        output.push_str(byte_count.to_string().as_str());
        output.push_str(" ");
    }

    if lines == true {
        let line_count = input.lines().count();
        output.push_str(line_count.to_string().as_str());
        output.push_str(" ");
    }

    if file_path.is_some() {
        let foo = file_path.unwrap();
        let bar = foo.to_str().unwrap();
        output.push_str(bar);
    }

    Ok(output)
}

/// wc - word, line, character, and byte count
#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short = 'c', help = "Output the bytes in a file")]
    pub bytes: bool,

    #[clap(short = 'l', help = "Output the lines in a file")]
    pub lines: bool,

    pub file_path: Option<PathBuf>,
}

pub fn get_args() -> Args {
    let args = Args::parse();

    args
}

pub fn parse_input(file_path: &Option<PathBuf>) -> Result<String, std::io::Error> {
    if let Some(path) = file_path {
        let file_contents = fs::read_to_string(path)?;
        return Ok(file_contents);
    }

    todo!("read from stdin");
}

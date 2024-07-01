use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;

pub fn build_output() -> Result<String, std::io::Error> {
    let Args {
        bytes,
        lines,
        words,
        characters,
        file_path,
    } = get_args();

    let default = !bytes && !lines && !words && !characters;
    let input = parse_input(&file_path)?;
    let mut output = String::new();

    if lines == true || default {
        let line_count = input.lines().count();

        output.push_str(line_count.to_string().as_str());
        output.push_str(" ");
    }

    if words == true || default {
        let mut word_count = 0;

        for line in input.lines() {
            word_count += line.split_whitespace().count();
        }

        output.push_str(word_count.to_string().as_str());
        output.push_str(" ");
    }

    if bytes == true || default {
        let byte_count = input.len();

        output.push_str(byte_count.to_string().as_str());
        output.push_str(" ");
    }

    if characters == true && !default {
        let character_count = input.chars().count();

        output.push_str(character_count.to_string().as_str());
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
    #[arg(short = 'c', help = "Output the number of bytes in a file")]
    pub bytes: bool,

    #[arg(short = 'l', help = "Output the number of lines in a file")]
    pub lines: bool,

    #[arg(short = 'w', help = "Output the number of words in a file")]
    pub words: bool,

    #[arg(short = 'm', help = "Output the number of characters in a file")]
    pub characters: bool,

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

    let mut stdin_input_buffer = String::new();

    for line_result in io::stdin().lines() {
        let line = line_result?;

        stdin_input_buffer.push_str(&line);
        stdin_input_buffer.push_str("\n");
    }

    Ok(stdin_input_buffer)
}

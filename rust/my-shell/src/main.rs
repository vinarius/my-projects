use std::io::{self, Write};
use std::path::Path;
use std::{env, fs};
use std::{process, str::FromStr};

enum BuiltInCommand {
    Echo,
    Exit,
    r#Type,
}

impl FromStr for BuiltInCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "echo" => Ok(BuiltInCommand::Echo),
            "exit" => Ok(BuiltInCommand::Exit),
            "type" => Ok(BuiltInCommand::Type),
            _ => Err(()),
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim_end().to_string();

        let mut input_split_whitespace_iter = input.split_whitespace();
        let command_maybe = input_split_whitespace_iter.next();

        if command_maybe.is_none() {
            continue;
        }

        let command_raw = command_maybe.unwrap();
        let command_parsed_maybe = command_raw.parse::<BuiltInCommand>();

        if command_parsed_maybe.is_err() {
            println!("{}: command not found", command_raw);
            continue;
        }

        let command_parsed =
            command_parsed_maybe.expect("expected to unwrap successfully parsed command");

        let options = input_split_whitespace_iter;

        match command_parsed {
            BuiltInCommand::Echo => {
                handle_echo_command(options);
                continue;
            }
            BuiltInCommand::Exit => {
                handle_exit_command(options);
                continue;
            }
            BuiltInCommand::Type => {
                handle_type_command(options);
                continue;
            }
        }
    }
}

fn handle_echo_command(options: std::str::SplitWhitespace) {
    let options_vec: Vec<_> = options.map(String::from).collect();
    let joined_string = options_vec.join(" ");

    println!("{joined_string}");
}

fn handle_exit_command(mut options: std::str::SplitWhitespace) {
    let exit_code_maybe = options.next();

    if exit_code_maybe.is_none() {
        return;
    }

    let exit_code = exit_code_maybe.expect("could not unwrap command option");

    if exit_code == "0" {
        process::exit(0);
    }
}

fn handle_type_command(mut options: std::str::SplitWhitespace) -> Result<(), io::Error> {
    let command_option_maybe = options.next();

    if command_option_maybe.is_none() {
        return Ok(());
    }

    let command_option_raw = command_option_maybe.expect("could not unwrap command option");
    let command_option_parsed = command_option_raw.parse::<BuiltInCommand>();

    if command_option_parsed.is_ok() {
        println!("{command_option_raw} is a shell builtin");

        return Ok(());
    }

    let path_maybe = env::var("PATH");

    if let Err(error) = path_maybe {
        eprintln!("{error}");

        return Ok(());
    }

    let path = path_maybe.unwrap();

    println!("path: {path:#?}");

    let path_split_by_directories = path.split(':');

    for directory in path_split_by_directories {
        let path = Path::new(directory);

        if !path.is_dir() {
            continue;
        }

        fs::read_dir(path)
    }

    println!("{command_option_raw}: not found");

    Ok(())
}

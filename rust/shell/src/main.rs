use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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
        let options = input_split_whitespace_iter;

        if command_maybe.is_none() {
            continue;
        }

        let command_raw = command_maybe.unwrap();
        let command_parsed_into_builtin_maybe = command_raw.parse::<BuiltInCommand>();

        if command_parsed_into_builtin_maybe.is_err() {
            run_external_program(command_raw, options);
            continue;
        }

        let command_parsed = command_parsed_into_builtin_maybe
            .expect("expected to unwrap successfully parsed command");

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

fn handle_type_command(mut options: std::str::SplitWhitespace) {
    let command_option_maybe = options.next();

    if command_option_maybe.is_none() {
        return;
    }

    let command_option_raw = command_option_maybe.expect("could not unwrap command option");
    let command_option_parsed = command_option_raw.parse::<BuiltInCommand>();

    if command_option_parsed.is_ok() {
        println!("{command_option_raw} is a shell builtin");

        return;
    }

    let command_path_maybe = get_command_path(command_option_raw);

    if command_path_maybe.is_none() {
        println!("{command_option_raw}: not found");
        return;
    }

    let command_path = command_path_maybe.unwrap();

    println!("{}", command_path.display());
}

fn run_external_program(command: &str, _options: std::str::SplitWhitespace) {
    let command_path_maybe = get_command_path(command);

    if command_path_maybe.is_none() {
        println!("{command}: not found");
        return;
    }

    // command is valid, not builtin, and in PATH

    let _command_path = command_path_maybe.unwrap();

    todo!("spawn a child process");
}

fn get_command_path(command: &str) -> Option<PathBuf> {
    let path_maybe = env::var("PATH");

    if let Err(error) = path_maybe {
        eprintln!("{error}");

        return None;
    }

    let path = path_maybe.unwrap();
    let path_split_by_directories = path.split(':');

    for directory in path_split_by_directories {
        let command_path = Path::new(directory).join(command);

        if command_path.exists() {
            return Some(command_path);
        }
    }

    None
}

use std::io::{self, Write};

use builtins::echo::handle_echo_command;
use builtins::exit::handle_exit_command;
use builtins::r#type::handle_type_command;
use enums::built_in_command::BuiltInCommand;
use utils::try_external_program;

mod builtins;
mod enums;
mod utils;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

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
            let _ = try_external_program(command_raw, options);
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

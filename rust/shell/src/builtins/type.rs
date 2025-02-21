use crate::{enums::built_in_command::BuiltInCommand, utils::get_command_path};

pub fn handle_type_command(mut options: std::str::SplitWhitespace) {
    let command_maybe = options.next();

    if command_maybe.is_none() {
        return;
    }

    let command_raw = command_maybe.expect("could not unwrap command option");
    let command_parsed = command_raw.parse::<BuiltInCommand>();

    if command_parsed.is_ok() {
        println!("{command_raw} is a shell builtin");

        return;
    }

    let command_path_maybe = get_command_path(command_raw);

    if command_path_maybe.is_none() {
        println!("{command_raw}: not found");
        return;
    }

    let command_path = command_path_maybe.unwrap();

    println!("{}", command_path.display());
}

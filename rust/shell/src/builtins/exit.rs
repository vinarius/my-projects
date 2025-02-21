use std::process;

pub fn handle_exit_command(mut options: std::str::SplitWhitespace) {
    let exit_code_maybe = options.next();

    if exit_code_maybe.is_none() {
        return;
    }

    let exit_code = exit_code_maybe.expect("could not unwrap command option");

    if exit_code == "0" {
        process::exit(0);
    }
}

pub fn handle_echo_command(options: std::str::SplitWhitespace) {
    let options_vec: Vec<_> = options.map(String::from).collect();
    let joined_string = options_vec.join(" ");

    println!("{joined_string}");
}

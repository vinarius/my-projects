mod logic;

fn main() {
    let output_result = logic::build_output();

    if let Err(err) = output_result {
        eprintln!("{err}");
        std::process::exit(1);
    }

    let output = output_result.unwrap();

    println!("{output}");
}

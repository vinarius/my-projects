use get_args::{get_args, Args};
use parse_input::parse_input;

mod get_args;
mod parse_input;

fn main() -> Result<(), std::io::Error> {
    let Args { bytes, file_path } = get_args();

    let input = parse_input(&file_path)?;

    if bytes == true {
        let byte_count = input.len();
        let mut output = format!("{byte_count}");

        if file_path.is_some() {
            output.push(' ');
            let foo = file_path.unwrap();
            let bar = foo.to_str().unwrap();
            output.push_str(bar);
        }

        println!("{output}");
    }

    return Ok(());
}

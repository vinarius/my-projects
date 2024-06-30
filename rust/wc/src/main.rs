use std::env;

#[derive(Debug)]
struct Args {
    bytes: bool,
}

fn main() {
    let mut args = Args { bytes: false };

    for arg in env::args() {
        if arg == "-c" {
            args.bytes = true;
        }
    }

    println!("{args:?}");
}

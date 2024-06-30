use std::env;

#[derive(Debug)]
pub struct Args {
    pub bytes: bool,
}

pub fn get_args() -> Args {
    let mut args = Args { bytes: false };

    for arg in env::args() {
        if arg == "-c" {
            args.bytes = true;
        }
    }

    args
}

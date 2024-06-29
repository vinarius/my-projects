use clap::Parser;

/// Print newline, word, and byte counts for the given file,
/// or read from stdin if no file is specified.
#[derive(Parser)]
struct MyArgs {
    /// Print the number of bytes
    #[arg(short)]
    c: (),
}

fn main() {
    let _args = MyArgs::parse();
    println!("Hello, world!");
}

use std::env;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

pub fn print_helo() {
    eprintln!("{}", "Test".green())
}

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        eprintln!("{} need to have 4 arguments", "error".red());
    }
}

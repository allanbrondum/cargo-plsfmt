use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let content = fs::read_to_string("example.txt").unwrap();
    let file = syn::parse_file(&content).unwrap();

    prettyplease::unparse(&file);
}
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a file to read as the first argument");
        return;
    }
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Could not read file");

    ukiyo::compile(contents);
}

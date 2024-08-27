use std::fs;
use regex::Regex;

fn main() {
    let path = "/Users/ymin/Downloads/AdventOfCode/AdventOfCodeDay9/input.txt";
    let contents = load_file(path);
    println!("{}", contents);
}

fn load_file(path: &str) -> String {
    // Load data from .txt format
    let file_path: String = String::from(path);
    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}

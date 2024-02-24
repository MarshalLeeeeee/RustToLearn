use std::env;
use std::fs;

fn test_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn test_file_io() {
    let content = fs::read_to_string("test_file").expect("test_file should exist");
    println!("{}", content);
}

fn main() {
    test_args();
    test_file_io();
}

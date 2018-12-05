use std::io;
use std::fs;

fn day1(f: &str) -> i32 {
    f.lines().map(|v: &str| -> i32 {v.parse().expect("not an int")}).sum()
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

fn main() {
    let f = read_file("input/input").expect("Cannot read file");
    println!("day1: {}",day1(&f));
}

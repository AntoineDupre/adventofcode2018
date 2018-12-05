use std::io;
use std::fs;
use std::collections::HashSet;


fn resulting_frequency(f: &str) -> i32 {
    f.lines().map(|v: &str| -> i32 {v.parse().expect("not an int")}).sum()
}


fn frequency(f: &str){
    let mut s = HashSet::new();
    let mut current = 0;
    for l in f.lines().cycle().map(|v: &str| -> i32 {v.parse().expect("not an int")}){
        current += l;
        if s.contains(&current){
            println!("{}", current);
            break;
        };
        s.insert(current);
    };
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}



fn main() {
    let f = read_file("input/input").expect("Cannot read file");
    println!("Resulting frequency {}",resulting_frequency(&f));
    println!("First frequency reached twice:");
    frequency(&f);
}

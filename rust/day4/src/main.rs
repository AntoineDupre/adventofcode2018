extern crate chrono;

use std::fs;
use chrono::{DateTime, Utc, TimeZone};

fn parse_input(f: &str) -> Vec<(DateTime<Utc>, String)>{
    // Build a vector of entries like (Date, Action)
    let mut output: Vec<(DateTime<Utc>, String)> = Vec::new();
    for line in f.lines(){
        // Date is always the same lenght, lets take advantage of it
        let date = Utc.datetime_from_str(&line[1..17], "%Y-%m-%d %H:%M").unwrap();
        let action = line[19..].to_string();
        output.push((date, action));
    }
    // Sort by date
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
    output.sort_by(|(date1, _), (date2, _)| date1.cmp(date2));
    // return parsed and sorted data set
    output
}


fn main() {
    let f = fs::read_to_string("input/input").expect("Cannot read file");
    let _data_set = parse_input(&f);
    //for data in data_set{
//	println!("{:?}", data);
  //  }
}

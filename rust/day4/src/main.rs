extern crate chrono;
extern crate regex;

use std::collections::HashMap;
use std::fs;
use std::boxed::Box;
// use chrono::{DateTime, Utc, Utc, TimeZone};

use chrono::prelude::*;

use regex::Regex;

#[derive(Default)]
struct Guard{
    id: i32,
    fall_sleeping: i32,
    sleeping_minutes: Vec<i32>,
}


impl Guard{
    fn start_sleeping(& mut self,  minute: i32){
        self.fall_sleeping = minute;
    }
    fn stop_sleeping(& mut self, minute: i32){
        if self.id == 971{
            println!("{} {} {}", self.fall_sleeping, minute, minute - self.fall_sleeping )
        }
	self.sleeping_minutes.push(minute - self.fall_sleeping);
    }


    fn max_sleeping(&self) -> &i32{
	self.sleeping_minutes.iter().max().unwrap()
    }
    fn total_sleeping(&self) -> i32{
        self.sleeping_minutes.iter().sum()
    }

}


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

fn build_guards(input: Vec<(DateTime<Utc>, String)>){
    // https://doc.rust-lang.org/std/boxed/struct.Box.html
    // let mut guards: Vec<Box<Guard>> = Vec::new();  //can not use a vec
    let mut guards: HashMap<i32, Box<Guard>> = HashMap::new();
    let mut id = 0;
    input.iter().for_each(|(d, action)|{
        // New guard
        //println!("{} {}", d, action);
        if action.contains("begins shift"){
            let re = Regex::new(r"(?P<id>\d+)").unwrap();
            let caps = re.captures(action).unwrap();
	    id = caps["id"].to_string().parse::<i32>().unwrap();
            guards.entry(id).or_insert(Box::new(Guard{id: id, fall_sleeping: -1, sleeping_minutes: Vec::new()}));
            // println!("{:?}", id);
	} else if action.contains("falls"){
            if let Some(guard) = guards.get_mut(&id){
                // let current_guard = guards.last_mut();
                guard.start_sleeping(d.minute() as i32);}
        } else if action.contains("wakes"){
            if let Some(guard) = guards.get_mut(&id){
                guard.stop_sleeping(d.minute() as i32);}
        }
    });
    // println!("{:?}", guards.iter().map(|g| g.total_sleeping()).max());
    let mut max = 0;
    let mut id = 0;
    for g in guards.values(){
        //println!("----");
        //println!("{} {} {} {}", g.id, g.total_sleeping(), g.max_sleeping(), max);
        if max < g.total_sleeping(){
            id = g.id;
            max = g.total_sleeping();
            println!("{} {} {} {:?}",id, g.max_sleeping(), max, g.sleeping_minutes);
        }
    }
}


fn main() {
    let f = fs::read_to_string("input/input").expect("Cannot read file");
    let _data_set = parse_input(&f);
    build_guards(_data_set);
}

use std::collections::HashMap;
use std::io;
use std::fs;


struct CharHistogram{
    char_distribution: HashMap<char, i32>
}


impl CharHistogram{
    pub fn new(word: &str) -> CharHistogram{
        let mut char_distribution = HashMap::new();
        for ch in word.chars(){
            // quiet cool: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
            let count = char_distribution.entry(ch).or_insert(0);
            *count += 1;
        }
        CharHistogram { char_distribution }
    }

    fn contains_value(&self, num: i32) -> bool{
        self.char_distribution.values().any(|&x| x == num)
    }

    pub fn has_two(&self) -> bool{
       self.contains_value(2)
    }

    pub fn has_three(&self) -> bool{
       self.contains_value(3)
    }
}


fn calculate_checksum(f: &str) -> i32{
    let mut count_twos = 0;
    let mut count_threes = 0;
    let histograms = f.lines().map(|word|CharHistogram::new(word));
    // Can probably merge with the above one
    for hist in histograms{
        if hist.has_two(){
            count_twos += 1;
        }
        if hist.has_three(){
            count_threes += 1;
        }
    };
    count_twos * count_threes
}


fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}


fn main() {
    let f = read_file("input/input").expect("Cannot read file");
    println!("Hash code {}" , calculate_checksum(&f));
}

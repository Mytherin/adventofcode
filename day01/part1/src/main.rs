use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut prev_int = 0;
    let mut count = 0;
    for line in lines {
        let current_int: i32 = line.unwrap().parse().unwrap();
        if current_int > prev_int {
            count += 1;
        }
        prev_int = current_int;
    }
    // -1 because the first does not count
    println!("{}", count - 1);
}

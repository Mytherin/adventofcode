use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut v: Vec<i32> = Vec::new();
    for line in lines {
        let current_int: i32 = line.unwrap().parse().unwrap();
        v.push(current_int);
    }
    let mut prev_window = 0;
    let mut count = 0;
    for i in 0..3 {
        prev_window += v[i];
    }
    for i in 3..v.len() {
        let current_window = prev_window + v[i] - v[i - 3];
        if current_window > prev_window {
            count += 1;
        }
        prev_window = current_window;
    }
    println!("{}", count);
}

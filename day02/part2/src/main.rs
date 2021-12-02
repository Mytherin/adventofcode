use std::fs::File;
use std::io::{self, BufRead};

// forward, down, up
fn main() {
    let mut hpos = 0;
    let mut depth = 0;
    let mut aim = 0;
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    for line in lines {
        let text = line.unwrap();
        let splits : Vec<&str> = text.split(' ').collect();
        let amount : i32 = splits[1].parse().unwrap();
        if splits[0] == "forward" {
            hpos += amount;
            depth += aim * amount;
        } else if splits[0] == "up" {
            aim -= amount;
        } else if splits[0] == "down" {
            aim += amount;
        } else {
            panic!("Unknown input {}", splits[0]);
        }
    }
    println!("{}", hpos * depth);
}

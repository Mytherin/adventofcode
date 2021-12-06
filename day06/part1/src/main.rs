use std::fs::File;
use std::io::{self, BufRead};

const DAY_COUNT : usize = 80;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut fishies : Vec<usize> = Vec::new();
    // parse
    for line in lines {
        let unwrapped = line.unwrap();
        let splits = unwrapped.split(",");
        for split in splits {
            fishies.push(split.parse().unwrap());
        }
    }
    for i in 0..DAY_COUNT {
        let current_fish_count = fishies.len();
        for fish_idx in 0..current_fish_count {
            if fishies[fish_idx] == 0 {
                fishies[fish_idx] = 6;
                fishies.push(8);
            } else {
                fishies[fish_idx] -= 1;
            }
        }
    }
    println!("{}", fishies.len());

}

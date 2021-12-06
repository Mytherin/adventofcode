use std::fs::File;
use std::io::{self, BufRead};

const DAY_COUNT : usize = 256;
const MAX_FISH_COUNT : usize = 9;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut fishies : Vec<usize> = Vec::new();
    for _i in 0..MAX_FISH_COUNT {
        fishies.push(0);
    }

    // parse
    for line in lines {
        let unwrapped = line.unwrap();
        let splits = unwrapped.split(",");
        for split in splits {
            let fish_count : usize = split.parse().unwrap();
            fishies[fish_count] += 1;
        }
    }
    for _day in 0..DAY_COUNT {
        let mut new_fishies : Vec<usize> = Vec::new();
        for _i in 0..MAX_FISH_COUNT {
            new_fishies.push(0);
        }
        for i in 0..MAX_FISH_COUNT {
            if i == 0 {
                new_fishies[6] += fishies[i];
                new_fishies[8] += fishies[i];
            } else {
                new_fishies[i - 1] += fishies[i];
            }
        }
        fishies = new_fishies;
    }
    let mut total_fish = 0;
    for i in 0..MAX_FISH_COUNT {
        total_fish += fishies[i];
    }
    println!("{}", total_fish);
}

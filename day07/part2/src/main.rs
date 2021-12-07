use std::fs::File;
use std::io::{self, BufRead};

fn abs_difference(x: usize, y: usize) -> usize {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn fuel_cost(x : usize) -> usize {
    // this can be done more efficiently
    // but it's 7AM and I don't want to think about it
    let mut sum = 0;
    for i in 0..x+1 {
        sum += i;
    }
    return sum;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut individual_crabs : Vec<usize> = Vec::new();
    let mut max_position = 0;
    // parse
    for line in lines {
        let unwrapped = line.unwrap();
        let splits = unwrapped.split(",");
        for split in splits {
            let crab_position : usize = split.parse().unwrap();
            if crab_position > max_position {
                max_position = crab_position;
            }
            individual_crabs.push(crab_position);
        }
    }
    max_position += 1;
    // construct a count of each crab
    let mut crabs : Vec<usize> = vec![0; max_position];
    for crab_position in individual_crabs {
        crabs[crab_position] += 1;
    }

    let mut cheapest_cost : usize = 9999999999999999;
    for hposition in 0..max_position {
        let mut cost = 0;
        for i in 0..max_position {
            cost += crabs[i] * fuel_cost(abs_difference(i, hposition));
        }
        if cost < cheapest_cost {
            cheapest_cost = cost;
        }
    }
    println!("{}", cheapest_cost);
}

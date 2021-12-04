use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut v: Vec<String> = Vec::new();
    for line in lines {
        v.push(line.unwrap());
    }
    let byte_count = v[0].len();
    let mut bins : Vec<i32> = vec![0; byte_count];
    // count the frequency of each of the bytes
    for line in v {
        if line.len() != byte_count {
            panic!("Unaligned lengths in input");
        }
        for (i, character) in line.chars().enumerate() {
            if character == '1' {
                bins[i] += 1;
            } else {
                bins[i] -= 1;
            }
        }
    }
    let mut gamma : u32 = 0;
    let mut epsilon : u32 = 0;
    for i in 0..byte_count {
        if bins[i] >= 0 {
            // 1 is most frequent: gamma is 1 at this point
            gamma = gamma | (1 << (byte_count - i - 1));
        } else {
            // 0 is most frequent: epsilon is 1 at this point
            epsilon = epsilon | (1 << (byte_count - i - 1));
        }
    }
    println!("{}", gamma * epsilon);
}

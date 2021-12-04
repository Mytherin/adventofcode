// To find oxygen generator rating, determine the most common value (0 or 1)
// in the current bit position, and keep only numbers with that bit in that
// position. If 0 and 1 are equally common, keep values with a 1 in
// the position being considered.
// To find CO2 scrubber rating, determine the least common value (0 or 1)
// in the current bit position, and keep only numbers with that bit in that
// position. If 0 and 1 are equally common, keep values with a 0 in the
// position being considered.

use std::fs::File;
use std::io::{self, BufRead};

fn parse_number(line : &str) -> u32 {
    let mut result : u32 = 0;
    let str_length = line.len();
    for (i, character) in line.chars().enumerate() {
        if character == '1' {
            result |= 1 << (str_length - i - 1);
        }
    }
    return result;
}

fn find_most_frequent_number(set : &Vec<&String>, byte_pos : usize, invert : bool) -> u32 {
    if set.len() == 1 {
        return parse_number(&set[0]);
    }
    // figure out the most common byte
    let mut count = 0;
    for line in set {
        if line.as_bytes()[byte_pos] == 49 {
            count += 1;
        } else {
            count -= 1;
        }
    }
    // now partition into the two sets
    let mut new_set: Vec<&String> = Vec::new();
    for line in set {
        if line.as_bytes()[byte_pos] == 49 {
            if count >= 0 && !invert {
                new_set.push(line);
            } else if count < 0 && invert {
                new_set.push(line);
            }
        } else {
            if count < 0 && !invert {
                new_set.push(line);
            } else if count >= 0 && invert {
                new_set.push(line);
            }
        }
    }
    if new_set.len() == 0 {
        panic!("panic! {}", set.len());
    }
    return find_most_frequent_number(&new_set, byte_pos + 1, invert);

}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut stored_strings: Vec<String> = Vec::new();
    let mut v: Vec<&String> = Vec::new();
    for line in lines {
        stored_strings.push(line.unwrap());
    }
    for line in &stored_strings {
        v.push(&line);
    }
    let mut oxygen = find_most_frequent_number(&v, 0, false);
    let mut co2 = find_most_frequent_number(&v, 0, true);
    println!("{}", oxygen * co2);
}

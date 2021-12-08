use std::fs::File;
use std::io::{self, BufRead};

const TOP : usize = 0;
const TOP_LEFT : usize = 1;
const TOP_RIGHT : usize = 2;
const MIDDLE : usize = 3;
const BOTTOM_LEFT : usize = 4;
const BOTTOM_RIGHT : usize = 5;
const BOTTOM : usize = 6;
const DIGIT_COUNT : usize = 10;

// len(2) => 1
// len(3) -> 7
// len(4) -> 4
// len(5) -> 2,3,5
// len(6) -> 0,6,9
// len(7) => 8

fn figure_out_codes(pattern_list : &Vec<String>, digits : &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    for digit in digits {
        if digit.len() == 2 {
            result.push(1);
        } else if digit.len() == 3 {
            result.push(7);
        } else if digit.len() == 4 {
            result.push(4);
        } else if digit.len() == 7 {
            result.push(8);
        } else if digit.len() == 5 {

        } else if digit.len() == 6 {

        } else {
            panic!("Unexpected length {}", digit.len());
        }
    }
    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut max_position = 0;
    // parse
    let mut digits_count : Vec<usize> = Vec::new();
    for i in 0..DIGIT_COUNT {
        digits_count.push(0);
    }
    for line in lines {
        let unwrapped = line.unwrap();
        let mut pattern_list : Vec<String> = Vec::new();
        let mut digits : Vec<String> = Vec::new();
        let mut current_pattern : String = String::new();
        let mut found_pipe = false;
        for character in unwrapped.chars() {
            if character == ' ' {
                // push the segment
                if current_pattern.len() == 0 {
                    continue;
                }
                if !found_pipe {
                    pattern_list.push(current_pattern);
                } else {
                    digits.push(current_pattern);
                }
                // move to the next segment
                current_pattern = String::new();
            } else if character == '|' {
                found_pipe = true;
                if pattern_list.len() != 10 {
                    panic!("Unexpected number of patterns ({})", pattern_list.len());
                }
            } else {
                current_pattern.push(character);
            }
        }
        digits.push(current_pattern);
        if digits.len() != 4 || pattern_list.len() != 10 {
            panic!("Unexpected number of digits ({}) or patterns ({})", digits.len(), pattern_list.len());
        }
        let decoded_digits = figure_out_codes(&pattern_list, &digits);
        for digit in decoded_digits {
            digits_count[digit] += 1;
        }
    }
    for i in 0..DIGIT_COUNT {
        println!("{}: {}", i, digits_count[i]);
    }
    println!("{}", digits_count[1] + digits_count[4] + digits_count[7] + digits_count[8]);
}

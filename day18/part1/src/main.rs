use std::fs::File;
use std::io::{self, BufRead};

const NUMBER_BEGIN : usize = 999999999;
const BRACKET_OPEN : usize = 1000000000;
const BRACKET_CLOSE : usize = 1000000001;
const COMMA : usize = 1000000002;

fn is_number(number : usize) -> bool {
    return number < NUMBER_BEGIN;
}

fn parse_collection(line : &String, collection : &mut Vec<usize>) {
    for character in line.chars() {
        if character == '[' {
            collection.push(BRACKET_OPEN);
        } else if character == ']' {
            collection.push(BRACKET_CLOSE);
        } else if character == ',' {
            collection.push(COMMA);
        } else {
            let number : usize = character.to_string().parse().unwrap();
            collection.push(number);
        }
    }
}


// addition:
// [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].

// explosion:
// If any pair is nested inside four pairs, the leftmost such pair explodes.
// To explode a pair, the pair's left value is added to the first regular number
// to the left of the exploding pair (if any), and the pair's right value is
// added to the first regular number to the right of the exploding pair (if any).
// Then, the entire exploding pair is replaced with the regular number 0.

// If any regular number is 10 or greater, the leftmost such regular number splits.
// To split a regular number, replace it with a pair; the left element of the pair
// should be the regular number divided by two and rounded down,
// while the right element of the pair should be the regular number
// divided by two and rounded up. For example, 10 becomes [5,5], 11 becomes
// [5,6], 12 becomes [6,6], and so on.
fn add_collection(left : &mut Vec<usize>, right : &mut Vec<usize>) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();
    result.push(BRACKET_OPEN);
    for member in left {
        result.push(*member);
    }
    result.push(COMMA);
    for member in right {
        result.push(*member);
    }
    result.push(BRACKET_CLOSE);
    return result;
}

fn reduce_collection(result : &mut Vec<usize>) {
    // do the normalization
    loop {
        let mut found_changes = false;
        let mut depth : usize = 0;
        for i in 0..result.len() {
            if result[i] == BRACKET_OPEN {
                // explode
                depth += 1;
                if depth > 4 {
                    // println!("explode at {}!", i);
                    // need to explode this pair
                    // first find the next few numbers
                    let first_number = result[i + 1];
                    if result[i + 2] != COMMA {
                        panic!("this needs to be a comma");
                    }
                    let second_number = result[i + 3];
                    // add the first number to the
                    for k in 0..i {
                        if is_number(result[i - k]) {
                            result[i - k] += first_number;
                            break;
                        }
                    }
                    for k in i+4..result.len() {
                        if is_number(result[k]) {
                            result[k] += second_number;
                            break;
                        }
                    }
                    // replace the pair with a 0
                    result[i] = 0;
                    for _k in 0..4 {
                        result.remove(i + 1);
                    }
                    found_changes = true;
                    break;
                }
            } else if result[i] == BRACKET_CLOSE {
                depth -= 1;
            }
        }
        if !found_changes {
            for i in 0..result.len() {
                if is_number(result[i]) && result[i] >= 10 {
                    // println!("split at {}!", i);
                    // split
                    let left_number = result[i] / 2;
                    let mut right_number = result[i] / 2;
                    if result[i] % 2 != 0 {
                        right_number += 1;
                    }
                    result[i] = BRACKET_OPEN;
                    result.insert(i + 1, left_number);
                    result.insert(i + 2, COMMA);
                    result.insert(i + 3, right_number);
                    result.insert(i + 4, BRACKET_CLOSE);

                    found_changes = true;
                    break;
                }
            }
        }

        if !found_changes {
            break;
        }
    }
}

fn print_collection(input : &Vec<usize>) {
    let mut result : String = String::new();
    for member in input {
        if *member == BRACKET_OPEN {
            result.push('[');
        } else if *member == BRACKET_CLOSE {
            result.push(']');
        } else if *member == COMMA {
            result.push(',');
        } else {
            result += &member.to_string();
        }
    }
    println!("{}", result);
}

fn calculate_magnitude(collection : &Vec<usize>) -> usize {
    let mut result : Vec<usize> = Vec::new();
    for number in collection {
        result.push(*number);
    }
    while result.len() > 1 {
        for i in 0..result.len() - 3 {
            if result[i] == BRACKET_OPEN {
                if is_number(result[i + 1]) && result[i + 2] == COMMA && is_number(result[i + 3]) {
                    if result[i + 4] != BRACKET_CLOSE {
                        panic!("bracket close");
                    }
                    // pair
                    result[i] = result[i + 1] * 3 + result[i + 3] * 2;
                    result.remove(i + 1);
                    result.remove(i + 1);
                    result.remove(i + 1);
                    result.remove(i + 1);
                    break;
                }
            }
        }
    }
    return result[0];
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut collection : Vec<usize> = Vec::new();
    for line in buf_lines {
        let unwrapped_line = line.unwrap();
        let mut current : Vec<usize> = Vec::new();
        parse_collection(&unwrapped_line, &mut current);
        if collection.len() == 0 {
            collection = current;
        } else {
            collection = add_collection(&mut collection, &mut current);
        }
        reduce_collection(&mut collection);
    }
    let magnitude = calculate_magnitude(&collection);
    println!("{}", magnitude);
}

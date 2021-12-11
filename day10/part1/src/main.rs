use std::fs::File;
use std::io::{self, BufRead};

fn pop_stack(stack : &mut Vec<char>, expected : char) -> usize{
    if stack.len() == 0 {
        panic!("no opening character");
    }
    let last = stack[stack.len() - 1];
    stack.pop();
    if last == expected {
        return 0;
    }
    match expected {
        '(' => return 3,
        '[' => return 57,
        '{' => return 1197,
        '<' => return 25137,
        _ => panic!("unrecognized character"),
    }
}

fn check_line(line : &String) -> usize {
    let mut stack : Vec<char> = Vec::new();
    for character in line.chars() {
        let mut score = 0;
        match character {
        '{' | '[' | '(' | '<' => stack.push(character),
        '}' => score = pop_stack(&mut stack, '{'),
        ']' => score = pop_stack(&mut stack, '['),
        ')' => score = pop_stack(&mut stack, '('),
        '>' => score = pop_stack(&mut stack, '<'),
        _ => panic!("unrecognized character"),
        }
        if score != 0 {
            return score;
        }
    }
    return 0;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    // parse
    let mut score = 0;
    for line in lines {
        let unwrapped = line.unwrap();
        score += check_line(&unwrapped);
    }
    println!("{}", score);
}

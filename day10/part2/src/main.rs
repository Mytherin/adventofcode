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
    let mut score = 0;
    for character in line.chars() {
        match character {
        '{' | '[' | '(' | '<' => stack.push(character),
        '}' => score = pop_stack(&mut stack, '{'),
        ']' => score = pop_stack(&mut stack, '['),
        ')' => score = pop_stack(&mut stack, '('),
        '>' => score = pop_stack(&mut stack, '<'),
        _ => panic!("unrecognized character"),
        }
        if score != 0 {
            break;
        }
    }
    if score != 0 {
        // corrupted line: ignore
        return 0;
    }
    // incomplete line: have to close the stack
    while stack.len() > 0 {
        let last = stack[stack.len() - 1];
        stack.pop();
        score *= 5;
        match last {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => panic!("unrecognized character"),
        }
    }
    return score;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    // parse
    let mut scores : Vec<usize> = Vec::new();
    for line in lines {
        let unwrapped = line.unwrap();
        let score = check_line(&unwrapped);
        if score > 0 {
            scores.push(score);
        }
    }
    scores.sort_by(|a, b| a.cmp(b));
    if scores.len() % 2 == 0 {
        panic!("even number of scores!");
    }
    let score = scores[scores.len() / 2];
    println!("{}", score);
}

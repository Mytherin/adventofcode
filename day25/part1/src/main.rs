use std::fs::File;
use std::io::{self, BufRead};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum CucumberType {
    East,
    South,
    Empty
}

// then the south facing herd
fn copy_map(map : &Vec<Vec<CucumberType>>) -> Vec<Vec<CucumberType>> {
    let mut new_map : Vec<Vec<CucumberType>> = Vec::new();
    for yvec in map {
        let mut hline : Vec<CucumberType> = Vec::new();
        for ctype in yvec {
            hline.push(*ctype);
        }
        new_map.push(hline);
    }
    return new_map;
}

fn perform_step(map : &mut Vec<Vec<CucumberType>>) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut move_count : usize = 0;
    // east facing herd moves first
    let mut new_map = copy_map(&map);
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == CucumberType::East {
                // check if there is a spot to move
                let next_x = if x + 1 == width { 0 } else { x + 1 };
                if map[y][next_x] == CucumberType::Empty {
                    move_count += 1;
                    new_map[y][x] = CucumberType::Empty;
                    new_map[y][next_x] = CucumberType::East;
                }
            }
        }
    }
    *map = copy_map(&new_map);
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == CucumberType::South {
                // check if there is a spot to move
                let next_y = if y + 1 == height { 0 } else { y + 1 };
                if map[next_y][x] == CucumberType::Empty {
                    move_count += 1;
                    new_map[y][x] = CucumberType::Empty;
                    new_map[next_y][x] = CucumberType::South;
                }
            }
        }
    }
    *map = copy_map(&new_map);
    return move_count;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut map : Vec<Vec<CucumberType>> = Vec::new();
    for line in lines {
        let unwrapped = line.unwrap();
        let mut hline : Vec<CucumberType> = Vec::new();
        for character in unwrapped.chars() {
            let cucumber_type = match character {
            '>' => CucumberType::East,
            'v' => CucumberType::South,
            '.' => CucumberType::Empty,
            _ => panic!("eek")
            };
            hline.push(cucumber_type);
        }
        map.push(hline);
    }
    let mut step_count = 0;
    loop {
        step_count += 1;
        let move_count = perform_step(&mut map);
        if move_count == 0 {
            break;
        }
    }
    println!("{}", step_count);
}

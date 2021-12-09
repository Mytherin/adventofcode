use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut height_map : Vec<Vec<usize>> = Vec::new();
    // parse
    for line in lines {
        let mut height : Vec<usize> = Vec::new();
        let unwrapped = line.unwrap();
        for character in unwrapped.chars() {
            height.push(character.to_string().parse().unwrap());
        }
        height_map.push(height);
    }
    if height_map.len() == 0 {
        panic!("height map of size 0");
    }
    let width : usize = height_map[0].len();
    let height : usize = height_map.len();
    let mut low_points : Vec<usize> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if x > 0 && height_map[x][y] >= height_map[x - 1][y] {
                continue;
            }
            if y > 0 && height_map[x][y] >= height_map[x][y - 1] {
                continue;
            }
            if x < width - 1 && height_map[x][y] >= height_map[x + 1][y] {
                continue;
            }
            if y < height - 1 && height_map[x][y] >= height_map[x][y + 1] {
                continue;
            }
            low_points.push(height_map[x][y]);
        }
    }
    let mut risk = 0;
    for low_point in low_points {
        risk += low_point + 1;
    }
    println!("{}", risk);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::mem;

struct Line {
    x1 : usize,
    y1 : usize,
    x2 : usize,
    y2 : usize
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut lines : Vec<Line> = Vec::new();
    // parse
    for line in buf_lines {
        let unwrapped_line = line.unwrap();
        let splits : Vec<&str> = unwrapped_line.split(" -> ").collect();
        let start : Vec<&str> = splits[0].split(",").collect();
        let end : Vec<&str> = splits[1].split(",").collect();
        let x1 : usize = start[0].parse().unwrap();
        let y1 : usize = start[1].parse().unwrap();
        let x2 : usize = end[0].parse().unwrap();
        let y2 : usize = end[1].parse().unwrap();
        lines.push(Line {x1: x1, y1: y1, x2: x2, y2: y2});
    }
    // compute board size
    let mut max_x : usize = 0;
    let mut max_y : usize = 0;
    for line in &lines {
        if line.x1 > max_x {
            max_x = line.x1;
        }
        if line.x2 > max_x {
            max_x = line.x2;
        }
        if line.y1 > max_y {
            max_y = line.y1;
        }
        if line.y2 > max_y {
            max_y = line.y2;
        }
    }
    max_x += 1;
    max_y += 1;
    // initialize the board
    let mut board : Vec<Vec<usize>> = Vec::new();
    for _x in 0..max_x {
        let mut new_vec = Vec::new();
        for _y in 0..max_y {
            new_vec.push(0);
        }
        board.push(new_vec);
    }
    // handle all the lines
    for line in &lines {
        if line.x1 == line.x2 {
            // vertical line
            let mut smallest : usize = line.y1;
            let mut biggest : usize = line.y2;
            if smallest > biggest {
                mem::swap(&mut smallest, &mut biggest);
            }
            for y in smallest..biggest+1 {
                board[line.x1][y] += 1;
            }
        } else if line.y1 == line.y2 {
            // horizontal line
            let mut smallest : usize = line.x1;
            let mut biggest : usize = line.x2;
            if smallest > biggest {
                mem::swap(&mut smallest, &mut biggest);
            }
            for x in smallest..biggest+1 {
                board[x][line.y1] += 1;
            }
        } else {
            let mut smallest_x : usize = line.x1;
            let mut smallest_y : usize = line.y1;
            let mut biggest_x : usize = line.x2;
            let mut biggest_y : usize = line.y2;
            if smallest_y > biggest_y {
                mem::swap(&mut smallest_x, &mut biggest_x);
                mem::swap(&mut smallest_y, &mut biggest_y);
            }
            let mut x : usize = smallest_x;
            for y in smallest_y..biggest_y+1 {
                board[x][y] += 1;
                if y != biggest_y {
                    if smallest_x > biggest_x {
                        x -= 1;
                    } else {
                        x += 1;
                    }
                }
            }
        }
    }
    // get the result
    let mut result = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if board[x][y] > 1 {
                result += 1;
            }
        }
    }
    println!("{}", result);
}

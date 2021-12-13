use std::fs::File;
use std::io::{self, BufRead};

struct Point {
    x : usize,
    y : usize,
}

fn fold_map(fold : &Point, map : &mut Vec<Vec<bool>>, width : &mut usize, height : &mut usize) {
    if fold.x > 0 {
        // horizontal fold
        for x in fold.x..*width + 1 {
            for y in 0..*height+1 {
                if map[x][y] {
                    map[fold.x - (x - fold.x)][y] = true;
                    map[x][y] = false;
                }
            }
        }
        *width = fold.x;
    } else if fold.y > 0 {
        // vertical fold
        // horizontal fold
        for x in 0..*width + 1 {
            for y in fold.y..*height+1 {
                if map[x][y] {
                    map[x][fold.y - (y - fold.y)] = true;
                    map[x][y] = false;
                }
            }
        }
        *height = fold.y;
    } else {
        panic!("empty fold!?");
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut points : Vec<Point> = Vec::new();
    let mut folds : Vec<Point> = Vec::new();
    let mut width : usize = 0;
    let mut height : usize = 0;
    // parse
    for line in lines {
        let unwrapped = line.unwrap();
        if unwrapped.len() == 0 {
            continue;
        }
        if unwrapped.starts_with("fold along ") {
            let splits : Vec<&str> = unwrapped.split('=').collect();
            let location = splits[1].parse().unwrap();
            if unwrapped.starts_with("fold along x") {
                folds.push(Point { x: location, y : 0 });
            } else {
                folds.push(Point { x: 0, y : location });
            }
        } else {
            let splits : Vec<&str> = unwrapped.split(',').collect();
            let x = splits[0].parse().unwrap();
            let y = splits[1].parse().unwrap();
            if x > width {
                width = x;
            }
            if y > height {
                height = y;
            }
            points.push(Point { x : x, y : y });
        }
    }
    // create the empty map
    let mut map : Vec<Vec<bool>> = Vec::new();
    for _x in 0..width+1 {
        let mut vertical : Vec<bool> = Vec::new();
        for _y in 0..height+1 {
            vertical.push(false);
        }
        map.push(vertical);
    }
    for point in points {
        map[point.x][point.y] = true;
    }
    // perform the fold
    for fold in folds {
        fold_map(&fold, &mut map, &mut width, &mut height);
    }

    let mut num_points = 0;
    for x in 0..width+1 {
        for y in 0..height+1 {
            if map[x][y] {
                num_points += 1;
            }
        }
    }
    for y in 0..height+1 {
        let mut line : String = String::new();
        for x in 0..width+1 {
            if map[x][y] {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

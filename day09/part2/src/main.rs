use std::fs::File;
use std::io::{self, BufRead};

struct Point {
    x : usize,
    y : usize
}

fn find_basin_size(
    x : usize,
    y : usize,
    width : usize,
    height : usize,
    height_map : &Vec<Vec<usize>>,
    visited : &mut Vec<Vec<bool>>) -> usize {
    if visited[x][y] || height_map[x][y] == 9 {
        return 0;
    }
    visited[x][y] = true;
    // visit the neighbors of this node recursively
    let mut score = 1;
    if x > 0 && height_map[x][y] < height_map[x - 1][y] {
        score += find_basin_size(x - 1, y, width, height, height_map, visited);
    }
    if y > 0 && height_map[x][y] < height_map[x][y - 1] {
        score += find_basin_size(x, y - 1, width, height, height_map, visited);
    }
    if x < width - 1 && height_map[x][y] < height_map[x + 1][y] {
        score += find_basin_size(x + 1, y, width, height, height_map, visited);
    }
    if y < height - 1 && height_map[x][y] < height_map[x][y + 1] {
        score += find_basin_size(x, y + 1, width, height, height_map, visited);
    }
    return score;
}

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
    let width : usize = height_map.len();
    let height : usize = height_map[0].len();
    let mut low_points : Vec<Point> = Vec::new();

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
            low_points.push(Point{x: x, y: y});
        }
    }
    let mut visited : Vec<Vec<bool>> = Vec::new();
    for _x in 0..width {
        let mut hvisited : Vec<bool> = Vec::new();
        for _y in 0..height {
            hvisited.push(false);
        }
        visited.push(hvisited);
    }

    let mut basin_sizes : Vec<usize> = Vec::new();
    for low_point in low_points {
        let basin_size = find_basin_size(low_point.x, low_point.y, width, height, &height_map, &mut visited);
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    let score = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    println!("{}", score);
}

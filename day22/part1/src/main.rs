use std::fs::File;
use std::io::{self, BufRead};

const MIN_POS : i64 = -50;
const MAX_POS : i64 = 50;

struct Range {
    turn_on : bool,
    xmin : i64,
    xmax : i64,
    ymin : i64,
    ymax : i64,
    zmin : i64,
    zmax : i64
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut ranges : Vec<Range>  = Vec::new();
    for line in buf_lines {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.len() == 0 {
            continue;
        }
        let mut xmin : i64 = 0;
        let mut xmax : i64 = 0;
        let mut ymin : i64 = 0;
        let mut ymax : i64 = 0;
        let mut zmin : i64 = 0;
        let mut zmax : i64 = 0;
        let splits : Vec<&str> = unwrapped_line.split(' ').collect();
        let turn_on = splits[0] == "on";
        let coordinates : Vec<&str> = splits[1].split(",").collect();
        for coordinate in coordinates {
            let coordinate_splits : Vec<&str> = coordinate.split('=').collect();
            let value_splits : Vec<&str> = coordinate_splits[1].split("..").collect();
            let min_value : i64 = value_splits[0].parse().unwrap();
            let max_value : i64 = value_splits[1].parse().unwrap();
            if coordinate_splits[0] == "x" {
                xmin = min_value;
                xmax = max_value;
            } else if coordinate_splits[0] == "y" {
                ymin = min_value;
                ymax = max_value;
            } else if coordinate_splits[0] == "z" {
                zmin = min_value;
                zmax = max_value;
            } else {
                panic!("Unknown coordinate split");
            }
        }
        ranges.push(Range { turn_on: turn_on, xmin: xmin, xmax: xmax, ymin: ymin, ymax: ymax, zmin: zmin, zmax: zmax});
    }
    let mut boxes : Vec<Vec<Vec<bool>>> = Vec::new();
    for _x in MIN_POS..MAX_POS+1 {
        let mut boxes_2d : Vec<Vec<bool>> = Vec::new();
        for _y in MIN_POS..MAX_POS+1 {
            let mut boxes_1d : Vec<bool> = Vec::new();
            for _z in MIN_POS..MAX_POS+1 {
                boxes_1d.push(false);
            }
            boxes_2d.push(boxes_1d);
        }
        boxes.push(boxes_2d);
    }
    for range in &mut ranges {
        if range.xmin < MIN_POS {
            range.xmin = MIN_POS;
        }
        if range.ymin < MIN_POS {
            range.ymin = MIN_POS;
        }
        if range.zmin < MIN_POS {
            range.zmin = MIN_POS;
        }
        if range.xmax > MAX_POS {
            range.xmax = MAX_POS;
        }
        if range.ymax > MAX_POS {
            range.ymax = MAX_POS;
        }
        if range.zmax > MAX_POS {
            range.zmax = MAX_POS;
        }
        for x in range.xmin..range.xmax+1 {
            for y in range.ymin..range.ymax+1 {
                for z in range.zmin..range.zmax+1 {
                    let x_index : usize = (x - MIN_POS).try_into().unwrap();
                    let y_index : usize = (y - MIN_POS).try_into().unwrap();
                    let z_index : usize = (z - MIN_POS).try_into().unwrap();
                    boxes[x_index][y_index][z_index] = range.turn_on;
                }
            }
        }
    }
    let mut count = 0;
    for x in MIN_POS..MAX_POS+1 {
        for y in MIN_POS..MAX_POS+1 {
            for z in MIN_POS..MAX_POS+1 {
                let x_index : usize = (x - MIN_POS).try_into().unwrap();
                let y_index : usize = (y - MIN_POS).try_into().unwrap();
                let z_index : usize = (z - MIN_POS).try_into().unwrap();
                if boxes[x_index][y_index][z_index] {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

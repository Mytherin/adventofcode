use std::fs::File;
use std::io::{self, BufRead};
use std::cmp;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Cube {
    turn_on : bool,
    xmin : i64,
    xmax : i64,
    ymin : i64,
    ymax : i64,
    zmin : i64,
    zmax : i64
}

// find the intersection region of two cubes (if any)
fn intersect(range_a : &Cube, range_b : &Cube) -> Option<Cube> {
    if range_a.xmin > range_b.xmax || range_b.xmin > range_a.xmax {
        return None;
    }
    if range_a.ymin > range_b.ymax || range_b.ymin > range_a.ymax {
        return None;
    }
    if range_a.zmin > range_b.zmax || range_b.zmin > range_a.zmax {
        return None;
    }
    // xmin: -23, xmax: 22, ymin: -17, ymax: 33, zmin: -1, zmax: 44
    // xmin: -25, xmax: 23, ymin: -10, ymax: 42, zmin: -38, zmax: 13
    // find the intersection range
    return Some(Cube {
        turn_on: true,
        xmin: cmp::max(range_a.xmin, range_b.xmin),
        xmax: cmp::min(range_a.xmax, range_b.xmax),
        ymin: cmp::max(range_a.ymin, range_b.ymin),
        ymax: cmp::min(range_a.ymax, range_b.ymax),
        zmin: cmp::max(range_a.zmin, range_b.zmin),
        zmax: cmp::min(range_a.zmax, range_b.zmax)
    });
}

fn split_cube_z(cube : &Cube, split_region : &Cube, xmin : i64, xmax : i64, ymin : i64, ymax : i64, result : &mut Vec<Cube>) {
    // split on z axis
    if split_region.zmin < cube.zmin || split_region.zmax > cube.zmax {
        panic!("no overlap on z axis");
    }

    if cube.zmin < split_region.zmin {
        // add cubes for [cube.zmin, split_region.zmin]
        result.push(Cube {
            turn_on: true,
            xmin: xmin,
            xmax: xmax,
            ymin: ymin,
            ymax: ymax,
            zmin: cube.zmin,
            zmax: split_region.zmin - 1
        });
    }
    // add cubes for split_region.ymin, split_region.ymax
    result.push(Cube {
        turn_on: true,
        xmin: xmin,
        xmax: xmax,
        ymin: ymin,
        ymax: ymax,
        zmin: split_region.zmin,
        zmax: split_region.zmax
    });

    if cube.zmax > split_region.zmax {
        // add cubes for [split_region.zmax, cube.zmax]
        result.push(Cube {
            turn_on: true,
            xmin: xmin,
            xmax: xmax,
            ymin: ymin,
            ymax: ymax,
            zmin: split_region.zmax + 1,
            zmax: cube.zmax
        });
    }
}

fn split_cube_y(cube : &Cube, split_region : &Cube, xmin : i64, xmax : i64, result : &mut Vec<Cube>) {
    // split on y axis
    if split_region.ymin < cube.ymin || split_region.ymax > cube.ymax {
        panic!("no overlap on y axis");
    }

    if cube.ymin < split_region.ymin {
        // add cubes for [cube.ymin, split_region.ymin]
        split_cube_z(cube, split_region, xmin, xmax, cube.ymin, split_region.ymin - 1, result);
    }
    // add cubes for split_region.ymin, split_region.ymax
    split_cube_z(cube, split_region, xmin, xmax, split_region.ymin, split_region.ymax, result);

    if cube.ymax > split_region.ymax {
        // add cubes for [split_region.ymax, cube.ymax]
        split_cube_z(cube, split_region, xmin, xmax, split_region.ymax + 1, cube.ymax, result);
    }
}

fn split_cube_x(cube : &Cube, split_region : &Cube, result : &mut Vec<Cube>) {
    // split on x axis
    if split_region.xmin < cube.xmin || split_region.xmax > cube.xmax {
        panic!("no overlap on x axis");
    }

    if cube.xmin < split_region.xmin {
        // add cubes for [cube.xmin, split_region.xmin]
        split_cube_y(cube, split_region, cube.xmin, split_region.xmin - 1, result);
    }
    // add cubes for split_region.xmin, split_region.xmax
    split_cube_y(cube, split_region, split_region.xmin, split_region.xmax, result);

    if cube.xmax > split_region.xmax {
        // add cubes for [split_region.xmax, cube.xmax]
        split_cube_y(cube, split_region, split_region.xmax + 1, cube.xmax, result);
    }
}

// split a cube into pieces, such that "split_region" is one of those regions (the last in the vector)
fn split_cube(cube : &Cube, split_region : &Cube) -> Vec<Cube> {
    let mut result : Vec<Cube> = Vec::new();
    split_cube_x(cube, split_region, &mut result);
    return result;
}

fn total_lit(cube : &Cube) -> usize {
    let count : i64 =
        ((cube.xmax + 1) - cube.xmin) *
        ((cube.ymax + 1) - cube.ymin) *
        ((cube.zmax + 1) - cube.zmin);
    return count.try_into().unwrap();
}

// fn print_range(range : &Cube) {
//     println!("xmin: {}, xmax: {}, ymin: {}, ymax: {}, zmin: {}, zmax: {}", range.xmin, range.xmax, range.ymin, range.ymax, range.zmin, range.zmax);
// }

fn push_new_cube_loop(new_cubes : &mut Vec<Cube>, cubes : &Vec<Cube>) -> bool {
    for i in 0..new_cubes.len() {
        for cube in &*cubes {
            match intersect(&cube, &new_cubes[i]) {
                Some(intersection) => {
                    let splits = split_cube(&new_cubes[i], &intersection);
                    new_cubes.remove(i);
                    for split in splits {
                        if split != intersection {
                            new_cubes.push(split);
                        }
                    }
                    return true;
                },
                None => ()
            };
        }
    }
    return false;
}

fn erase_cube_loop(remove_cube : &Cube, cubes : &mut Vec<Cube>) -> bool {
    for i in 0..cubes.len() {
        match intersect(remove_cube, &cubes[i]) {
            Some(intersection) => {
                let splits = split_cube(&cubes[i], &intersection);
                cubes.remove(i);
                for split in splits {
                    if split != intersection {
                        cubes.push(split);
                    }
                }
                return true;
            }
            None => ()
        };
    }
    return false;
}

fn push_new_cube(new_cube : &Cube, cubes : &mut Vec<Cube>) {
    if new_cube.turn_on {
        let mut new_cubes : Vec<Cube> = vec![new_cube.clone()];
        let mut found_intersection : bool = true;
        while found_intersection {
            found_intersection = push_new_cube_loop(&mut new_cubes, cubes);
        }
        for new_cube in new_cubes {
            cubes.push(new_cube);
        }
    } else {
        // turn off cubes
        let mut found_intersection : bool = true;
        while found_intersection {
            found_intersection = erase_cube_loop(new_cube, cubes);
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut ranges : Vec<Cube>  = Vec::new();
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
        ranges.push(Cube { turn_on: turn_on, xmin: xmin, xmax: xmax, ymin: ymin, ymax: ymax, zmin: zmin, zmax: zmax});
    }
    let mut cubes : Vec<Cube> = Vec::new();
    for range in &ranges {
        push_new_cube(range, &mut cubes);
    }

    let mut count : usize = 0;
    for cube in &cubes {
        count += total_lit(&cube);
    }
    println!("{}", count);
}

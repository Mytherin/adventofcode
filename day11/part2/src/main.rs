use std::fs::File;
use std::io::{self, BufRead};

struct Octopus {
    energy : usize,
    has_flashed : bool,
}

fn add_energy(octopus_map : &mut Vec<Vec<Octopus>>, x : usize, y : usize) {
    octopus_map[x][y].energy += 1;
    if octopus_map[x][y].has_flashed || octopus_map[x][y].energy <= 9 {
        // this octopus either already has flashed
        // or does not have enough energy to flash
        return;
    }
    // the octopus will flash!
    octopus_map[x][y].has_flashed = true;
    let left_edge = x == 0;
    let right_edge = x == (octopus_map[0].len() - 1);
    let top_edge = y == 0;
    let bottom_edge = y == octopus_map.len() - 1;
    // add energy to all adjacent neighbors
    // x - 1
    if !left_edge {
        add_energy(octopus_map, x - 1, y);
        if !top_edge {
            add_energy(octopus_map, x - 1, y - 1);
        }
        if !bottom_edge {
            add_energy(octopus_map, x - 1, y + 1);
        }
    }
    // x
    if !top_edge {
        add_energy(octopus_map, x, y - 1);
    }
    if !bottom_edge {
        add_energy(octopus_map, x, y + 1);
    }
    // x + 1
    if !right_edge {
        add_energy(octopus_map, x + 1, y);
        if !top_edge {
            add_energy(octopus_map, x + 1, y - 1);
        }
        if !bottom_edge {
            add_energy(octopus_map, x + 1, y + 1);
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut octopus_map : Vec<Vec<Octopus>> = Vec::new();
    // parse
    for line in lines {
        let mut octopus_hline : Vec<Octopus> = Vec::new();
        let unwrapped = line.unwrap();
        for character in unwrapped.chars() {
            let initial_energy = character.to_string().parse().unwrap();
            octopus_hline.push(Octopus { energy : initial_energy, has_flashed : false });
        }
        octopus_map.push(octopus_hline);
    }
    let width : usize = octopus_map[0].len();
    let height : usize = octopus_map.len();
    let mut step = 1;
    loop {
        // reset the flash of all octopus
        for x in 0..width {
            for y in 0..height {
                octopus_map[x][y].has_flashed = false;
            }
        }
        // now perform the actual energy increment
        for x in 0..width {
            for y in 0..height {
                add_energy(&mut octopus_map, x, y);
            }
        }
        // finally add up all the flashes that occurred
        // and reset the energy of any octopus that flashed to 0
        let mut flash_count = 0;
        for x in 0..width {
            for y in 0..height {
                if octopus_map[x][y].has_flashed {
                    flash_count += 1;
                    octopus_map[x][y].energy = 0;
                }
            }
        }
        if flash_count == width * height {
            break;
        }
        step += 1;
    }
    println!("{}", step);
}

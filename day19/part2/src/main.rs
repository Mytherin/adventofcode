use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

const MIN_OVERLAPPING_BEACONS : usize = 12;
const TOTAL_CONFIGURATIONS : usize = 24;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x : i32,
    y : i32,
    z : i32
}

struct Overlap {
    orientation : usize,
    xtranslate : i32,
    ytranslate : i32,
    ztranslate : i32
}

struct Scanner {
    index : usize,
    positions : Vec<Point>,
}

// orientation
// 24 different orientations
// x = x, x = -x, x = y, x = -y, x = z, x = -z
// y = y
// z = z
fn generate_orientation(x: i32, y: i32, z : i32, index : usize, result : &mut Vec<Vec<Point>>) {
    // FACING x
    result[index + 0].push(Point { x: x, y: y,  z: z});
    result[index + 1].push(Point { x: x, y: -z, z: y});
    result[index + 2].push(Point { x: x, y: -y, z: -z});
    result[index + 3].push(Point { x: x, y: z,  z: -y});

    // FACING -x
    result[index + 4].push(Point { x: -x, y: y, z: -z});
    result[index + 5].push(Point { x: -x, y: -z,  z: -y});
    result[index + 6].push(Point { x: -x, y: -y,  z: z});
    result[index + 7].push(Point { x: -x, y: z, z: y});
}

fn generate_configurations(positions : &Vec<Point>) -> Vec<Vec<Point>> {
    let mut result : Vec<Vec<Point>> = Vec::new();
    for _i in 0..TOTAL_CONFIGURATIONS {
        result.push(Vec::new());
    }
    for p in positions {
        // facing x
        generate_orientation(p.x, p.y, p.z, 0, &mut result);
        // facing y
        generate_orientation(p.y, -p.x, p.z, 8, &mut result);
        // facing z
        generate_orientation(p.z, p.y, -p.x, 16, &mut result);
    }
    return result;
}

fn translate_positions(xoffset : i32, yoffset : i32, zoffset : i32, positions : &Vec<Point>) -> Vec<Point> {
    let mut result : Vec<Point> = Vec::new();
    for p in positions {
        result.push(Point { x : p.x + xoffset, y : p.y + yoffset, z: p.z + zoffset});
    }
    return result;
}

fn find_overlap(apos : &Vec<Point>, bpos : &Vec<Point>) -> usize {
    let mut hash_map : HashSet<Point> = HashSet::new();
    for pos in apos {
        hash_map.insert(*pos);
    }

    let mut count = 0;
    for pos in bpos {
        if hash_map.contains(pos) {
            count += 1;
        }
    }
    return count;
}

fn translate_scanner(positions : &Vec<Point>, overlap_struct : &Overlap) -> Vec<Point> {
    let configurations = generate_configurations(positions);
    let points = &configurations[overlap_struct.orientation];
    return translate_positions(overlap_struct.xtranslate, overlap_struct.ytranslate, overlap_struct.ztranslate, &points);
}

fn find_overlapping_beacons(left : &Scanner, right : &Scanner) -> Option<Overlap> {
    let oriented_positions = generate_configurations(&left.positions);
    let mut beacon_overlap : usize = 0;
    let mut overlap_struct : Overlap = Overlap { orientation: 0, xtranslate: 0, ytranslate: 0, ztranslate: 0 };

    // try to find the offset
    for index in 0..oriented_positions.len() {
        let position_list = &oriented_positions[index];
        let mut orientation_overlap = 0;
        // try to fix the first point to a pointer in the right scanner
        for position_index in 0..position_list.len() {
            for fixed_index in 0..right.positions.len() {
                let position_diff_x = right.positions[fixed_index].x - position_list[position_index].x;
                let position_diff_y = right.positions[fixed_index].y - position_list[position_index].y;
                let position_diff_z = right.positions[fixed_index].z - position_list[position_index].z;
                let translated_positions = translate_positions(position_diff_x, position_diff_y, position_diff_z, &position_list);

                let overlap = find_overlap(&translated_positions, &right.positions);
                if overlap >= MIN_OVERLAPPING_BEACONS {
                    if orientation_overlap >= MIN_OVERLAPPING_BEACONS {
                        continue;
                    }
                    overlap_struct.orientation = index;
                    overlap_struct.xtranslate = position_diff_x;
                    overlap_struct.ytranslate = position_diff_y;
                    overlap_struct.ztranslate = position_diff_z;
                }
                if overlap > orientation_overlap {
                    orientation_overlap = overlap;
                }
            }
        }
        if orientation_overlap >= MIN_OVERLAPPING_BEACONS {
            if beacon_overlap != 0 {
                panic!("Multiple overlaps found!");
            }
            beacon_overlap = orientation_overlap;
        }
    }
    if beacon_overlap >= MIN_OVERLAPPING_BEACONS {
        return Some(overlap_struct);
    } else {
        return None;
    }
}

fn get_distance(a : Point, b : Point) -> i32 {
    return (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs();
}

// This region can be reconstructed by finding pairs of scanners that have overlapping detection regions such that there are at least 12 beacons that both scanners detect within the overlap. By establishing 12 common beacons, you can precisely determine where the scanners are relative to each other, allowing you to reconstruct the beacon map one scanner at a time.
fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut scanners : Vec<Scanner> = Vec::new();
    let mut current_scanner = Scanner { index: 0, positions : Vec::new() };
    let mut scanner_index = 0;
    for line in buf_lines {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.len() == 0 {
            continue;
        }
        if unwrapped_line.starts_with("---") {
            if current_scanner.positions.len() > 0 {
                scanners.push(current_scanner);
            }
            current_scanner = Scanner { index : scanner_index, positions : Vec::new() };
            scanner_index += 1;
        } else {
            let splits : Vec<&str> = unwrapped_line.split(",").collect();
            let x : i32 = splits[0].parse().unwrap();
            let y : i32 = splits[1].parse().unwrap();
            let z : i32 = splits[2].parse().unwrap();
            current_scanner.positions.push(Point { x: x, y: y, z: z });
        }
    }
    if current_scanner.positions.len() > 0 {
        scanners.push(current_scanner);
    }

    let mut translations : HashMap<usize, HashMap<usize, Overlap>> = HashMap::new();
    for xscanner in 0..scanners.len() {
        for yscanner in 0..scanners.len() {
            if xscanner == yscanner {
                continue;
            }
            let option = find_overlapping_beacons(&scanners[xscanner], &scanners[yscanner]);
            match option {
                Some(overlap) => {
                    println!("overlap in {} and {}", xscanner, yscanner);
                    let entry = translations.entry(xscanner).or_insert(HashMap::new());
                    entry.insert(yscanner, overlap);
                },
                _ => ()
            }
        }
    }

    let mut all_scanners : Vec<Point> = Vec::new();
    all_scanners.push(Point { x: 0, y: 0, z: 0 });
    // translate everything to scanner 0
    for xscanner in 1..scanners.len() {
        let mut handled_scanners : HashSet<usize> = HashSet::new();
        let mut candidate_scanners : Vec<Scanner> = Vec::new();
        let mut zero_positions : Vec<Point> = Vec::new();
        handled_scanners.insert(xscanner);
        // translate the position (0, 0, 0)
        zero_positions.push(Point { x: 0, y: 0, z: 0 });
        candidate_scanners.push(Scanner { index: xscanner, positions: zero_positions });
        let mut scanner_index = 0;
        let mut found = false;
        loop {
            if scanner_index >= candidate_scanners.len() {
                panic!("Not found!?!?!");
            }
            // find all candidate scanners that are not handled yet
            let current_index = candidate_scanners[scanner_index].index;
            let current_positions = candidate_scanners[scanner_index].positions.clone();
            for (key, overlap) in &translations[&current_index] {
                if handled_scanners.contains(key) {
                    continue;
                }
                handled_scanners.insert(*key);
                let translated = translate_scanner(&current_positions, overlap);
                if *key == 0 {
                    for point in translated {
                        all_scanners.push(point);
                    }
                    found = true;
                    break;
                }
                candidate_scanners.push(Scanner {
                    index: *key,
                    positions: translated
                });
            }
            if found {
                break;
            }
            scanner_index += 1;
        }
    }
    // figure out the max distance between points
    let mut max_distance = 0;
    for p in &all_scanners {
        for p2 in &all_scanners {
            let distance = get_distance(*p, *p2);
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    println!("{}", max_distance);
}

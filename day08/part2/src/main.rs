use std::fs::File;
use std::io::{self, BufRead};

const TOP : usize = 0;
const TOP_LEFT : usize = 1;
const TOP_RIGHT : usize = 2;
const MIDDLE : usize = 3;
const BOTTOM_LEFT : usize = 4;
const BOTTOM_RIGHT : usize = 5;
const BOTTOM : usize = 6;
const SIDE_COUNT : usize = 7;
const DIGIT_COUNT : usize = 10;

const NO_SIDE_REMAINING : usize = 10;
const MULTIPLE_SIDES_REMAINING : usize = 11;

// restrictions used to solve the puzzle:

// len(2) => 1
// len(3) -> 7
// len(4) -> 4
// len(5) -> 2,3,5
// len(6) -> 0,6,9
// len(7) => 8

// bottom_left is in 4 digits
// top_left is in 6 digits
// middle is in 7 digits
// bottom is in 7 digits
// top is in 8 digits
// top_right is in 8 digits
// bottom_right is in 9 digits

fn character_to_digit(character : char) -> usize {
    match character {
        'a' => return 0,
        'b' => return 1,
        'c' => return 2,
        'd' => return 3,
        'e' => return 4,
        'f' => return 5,
        'g' => return 6,
        _ => panic!("unrecognized character"),
    }
}

fn digit_to_character(index : usize) -> char {
    match index {
        0 => return 'a',
        1 => return 'b',
        2 => return 'c',
        3 => return 'd',
        4 => return 'e',
        5 => return 'f',
        6 => return 'g',
        _ => panic!("unrecognized character"),
    }
}

fn digit_to_side(index : usize) -> &'static str {
    match index {
        0 => return "TOP",
        1 => return "TOP_LEFT",
        2 => return "TOP_RIGHT",
        3 => return "MIDDLE",
        4 => return "BOTTOM_LEFT",
        5 => return "BOTTOM_RIGHT",
        6 => return "BOTTOM",
        _ => panic!("unrecognized side"),
    }
}

fn remove_candidate(index : usize, candidate : usize, candidates_per_letter : &mut Vec<Vec<bool>>) {
    candidates_per_letter[index][candidate] = false;
}

fn get_side_for_letter(index : usize, candidates_per_letter : &Vec<Vec<bool>>) -> usize {
    let mut result : usize = NO_SIDE_REMAINING;
    for side in 0..SIDE_COUNT {
        if candidates_per_letter[index][side] {
            if result == NO_SIDE_REMAINING {
                result = side;
            } else {
                result = MULTIPLE_SIDES_REMAINING;
            }
        }
    }
    if result == NO_SIDE_REMAINING {
        panic!("no sides remaining");
    }
    return result;
}

fn remove_impossible_sides(index : usize, impossible_sides : &Vec<usize>, candidates_per_letter : &mut Vec<Vec<bool>>) {
    // remove all the impossible sides from this character
    for candidate in impossible_sides {
        remove_candidate(index, *candidate, candidates_per_letter);
    }
    // check if there is only one candidate remaining
    let side_remaining = get_side_for_letter(index, candidates_per_letter);
    if side_remaining != MULTIPLE_SIDES_REMAINING {
        // only one side remains: remove this side as a candidate from all the other letters
        for other_index in 0..SIDE_COUNT {
            if other_index != index {
                remove_candidate(other_index, side_remaining, candidates_per_letter);
            }
        }
    }
}

fn remove_candidates(pattern : &String, potential_sides : Vec<usize>, impossible_sides : Vec<usize>, candidates_per_letter : &mut Vec<Vec<bool>>) {
    let mut character_indexes : Vec<usize> = Vec::new();
    // first remove all impossible sides
    for character in pattern.chars() {
        let index = character_to_digit(character);
        character_indexes.push(index);
        remove_impossible_sides(index, &impossible_sides, candidates_per_letter);
    }
    // now the potential sides need to be removed from the other characters
    for index in 0..SIDE_COUNT {
        if character_indexes.contains(&index) {
            continue;
        }
        // this character is not part of the pattern
        // it cannot have the potential sides
        for candidate in &potential_sides {
            remove_candidate(index, *candidate, candidates_per_letter);
        }
    }
}

fn print_candidates(candidates_per_letter : &Vec<Vec<bool>>) {
    // print candidates
    for letter in 0..SIDE_COUNT {
        println!("{}", digit_to_character(letter));
        for side in 0..SIDE_COUNT {
            if candidates_per_letter[letter][side] {
                println!("{}", digit_to_side(side));
            }
        }
    }
}

fn get_digit(digit : &String, candidates_per_letter : &Vec<Vec<bool>>) -> usize {
    // handle the remainder
    let mut number_sides : Vec<Vec<usize>> = Vec::new();
    // 0
    number_sides.push(vec![TOP, TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM]);
    // 1
    number_sides.push(vec![TOP_RIGHT, BOTTOM_RIGHT]);
    // 2
    number_sides.push(vec![TOP, TOP_RIGHT, MIDDLE, BOTTOM_LEFT, BOTTOM]);
    // 3
    number_sides.push(vec![TOP, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT, BOTTOM]);
    // 4
    number_sides.push(vec![TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT]);
    // 5
    number_sides.push(vec![TOP, TOP_LEFT, MIDDLE, BOTTOM_RIGHT, BOTTOM]);
    // 6
    number_sides.push(vec![TOP, TOP_LEFT, MIDDLE, BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM]);
    // 7
    number_sides.push(vec![TOP, TOP_RIGHT, BOTTOM_RIGHT]);
    // 8
    number_sides.push(vec![TOP, TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM]);
    // 9
    number_sides.push(vec![TOP, TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT, BOTTOM]);

    // get the sides that are there
    let mut sides : Vec<usize> = Vec::new();
    for character in digit.chars() {
        let index = character_to_digit(character);
        let side = get_side_for_letter(index, candidates_per_letter);
        if side == MULTIPLE_SIDES_REMAINING {
            print_candidates(candidates_per_letter);
            panic!("Multiple sides remaining for letter {}", character);
        }
        sides.push(side);
    }
    // sort the sides
    sides.sort_by(|a, b| a.cmp(b));
    // now do a comparison with the number sides to figure out the number
    for number in 0..DIGIT_COUNT {
        if sides == number_sides[number] {
            return number;
        }
    }
    for side in sides {
        println!("{}", digit_to_side(side));
    }
    panic!("Could not retrieve digit!");
}

fn figure_out_codes(pattern_list : &Vec<String>, digits : &Vec<String>) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();
    let mut candidates_per_letter : Vec<Vec<bool>> = Vec::new();
    for _i in 0..SIDE_COUNT {
        let mut candidates : Vec<bool> = Vec::new();
        for _side in 0..SIDE_COUNT {
            candidates.push(true);
        }
        candidates_per_letter.push(candidates);
    }
    // first we do some pruning of the potential candidates per letter
    // based on the restrictions we know of the pattern length
    for pattern in pattern_list {
        if pattern.len() == 2 {
            // 1, can only be TOP_RIGHT or BOTTOM_RIGHT
            remove_candidates(pattern, vec![TOP_RIGHT, BOTTOM_RIGHT], vec![TOP, TOP_LEFT, MIDDLE, BOTTOM_LEFT, BOTTOM], &mut candidates_per_letter);
        } else if pattern.len() == 3 {
            // 7, can only be TOP, TOP_RIGHT or BOTTOM_RIGHT
            remove_candidates(pattern, vec![TOP, TOP_RIGHT, BOTTOM_RIGHT], vec![TOP_LEFT, MIDDLE, BOTTOM_LEFT, BOTTOM], &mut candidates_per_letter);
        } else if pattern.len() == 4 {
            // 4, can only be TOP_LEFT, TOP_RIGHT, MIDDLE or BOTTOM_RIGHT
            remove_candidates(pattern, vec![TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT], vec![TOP, BOTTOM_LEFT, BOTTOM], &mut candidates_per_letter);
        }
    }
    // now we prune based on the frequency of the sides in the letters
    let mut letter_count : Vec<usize> = Vec::new();
    for _i in 0..SIDE_COUNT {
        letter_count.push(0);
    }
    for pattern in pattern_list {
        for character in pattern.chars() {
            let index = character_to_digit(character);
            letter_count[index] += 1;
        }
    }
    // bottom_left is in 4 letters
    // top_left is in 6 letters
    // middle is in 7 letters
    // bottom is in 7 letters
    // top is in 8 letters
    // top_right is in 8 letters
    // bottom_right is in 9 letters
    for i in 0..SIDE_COUNT {
        let count = letter_count[i];
        let mut possible_sides : Vec<usize> = Vec::new();
        if count == 4 {
            // has to be bottom left
            possible_sides.push(BOTTOM_LEFT);
        } else if count == 6 {
            // has to be top_left
            possible_sides.push(TOP_LEFT);
        } else if count == 7 {
            // has to be middle or bottom
            possible_sides.push(MIDDLE);
            possible_sides.push(BOTTOM);
        } else if count == 8 {
            // has to be top or top_right
            possible_sides.push(TOP);
            possible_sides.push(TOP_RIGHT);
        } else if count == 9 {
            // has to be bottom_right
            possible_sides.push(BOTTOM_RIGHT);
        }
        let mut impossible_sides : Vec<usize> = Vec::new();
        for side in 0..SIDE_COUNT {
            if !possible_sides.contains(&side) {
                impossible_sides.push(side);
            }
        }
        remove_impossible_sides(i, &impossible_sides, &mut candidates_per_letter);
    }
    for digit in digits {
        result.push(get_digit(digit, &candidates_per_letter));
    }
    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    // parse
    let mut sum : usize = 0;
    for line in lines {
        let unwrapped = line.unwrap();
        let mut pattern_list : Vec<String> = Vec::new();
        let mut digits : Vec<String> = Vec::new();
        let mut current_pattern : String = String::new();
        let mut found_pipe = false;
        for character in unwrapped.chars() {
            if character == ' ' {
                // push the segment
                if current_pattern.len() == 0 {
                    continue;
                }
                let mut chars : Vec<char> = current_pattern.chars().collect();
                chars.sort_by(|a, b| a.cmp(b));
                current_pattern = String::from_iter(chars);
                if !found_pipe {
                    pattern_list.push(current_pattern);
                } else {
                    digits.push(current_pattern);
                }
                // move to the next segment
                current_pattern = String::new();
            } else if character == '|' {
                found_pipe = true;
                if pattern_list.len() != 10 {
                    panic!("Unexpected number of patterns ({})", pattern_list.len());
                }
            } else {
                current_pattern.push(character);
            }
        }
        digits.push(current_pattern);
        if digits.len() != 4 || pattern_list.len() != 10 {
            panic!("Unexpected number of digits ({}) or patterns ({})", digits.len(), pattern_list.len());
        }
        let decoded_digits = figure_out_codes(&pattern_list, &digits);
        if decoded_digits.len() != 4 {
            panic!("Missing digits in result");
        }
        let mut result_number = 0;
        for digit in decoded_digits {
            result_number = result_number * 10 + digit;
        }
        sum += result_number;
    }
    println!("{}", sum);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

// [0N, NN, NC, CB, B0]

// NN -> NC, CN
fn add_pair(char_one : char, char_two : char, pair_count : &mut HashMap<char, HashMap<char, usize>>, count : usize) {
    *pair_count.entry(char_one).or_insert(HashMap::new()).entry(char_two).or_insert(0) += count;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut polymer : String = String::new();
    let mut transformation_rules : HashMap<char, HashMap<char, char>> = HashMap::new();
    let mut pair_count : HashMap<char, HashMap<char, usize>> = HashMap::new();

    // parse
    let mut first_line = true;
    for line in lines {
        let unwrapped = line.unwrap();
        if unwrapped.len() == 0 {
            continue;
        }
        if first_line {
            // parse the polymer
            first_line = false;
            polymer = unwrapped;
        } else {
            // parse the polymer transformation rule
            let splits : Vec<&str> = unwrapped.split(" -> ").collect();
            let chars_rule : Vec<char> = splits[0].chars().collect();
            let chars_result : Vec<char> = splits[1].chars().collect();
            if chars_rule.len() != 2 || chars_result.len() != 1 {
                panic!("Eek");
            }
            transformation_rules.entry(chars_rule[0]).or_insert(HashMap::new()).entry(chars_rule[1]).or_insert(chars_result[0]);
        }
    }
    // set up the initial pair counts
    let polymer_chars : Vec<char> = polymer.chars().collect();
    for i in 0..polymer_chars.len() {
        if i == 0 {
            add_pair('0', polymer_chars[i], &mut pair_count, 1);
        }
        if i + 1 == polymer_chars.len() {
            add_pair(polymer_chars[i], '0', &mut pair_count, 1);
        } else {
            add_pair(polymer_chars[i], polymer_chars[i + 1], &mut pair_count, 1);
        }
    }
    for _step in 0..40 {
        let mut new_pair_count : HashMap<char, HashMap<char, usize>> = HashMap::new();
        // iterate over all of the pairs
        for (first_character, hash_map) in &pair_count {
            for (second_character, count) in hash_map {
                let mut found = false;
                if transformation_rules.contains_key(&first_character) {
                    let val = &transformation_rules[&first_character];
                    if val.contains_key(&second_character) {
                        let result_character = val[&second_character];
                        add_pair(*first_character, result_character, &mut new_pair_count, *count);
                        add_pair(result_character, *second_character, &mut new_pair_count, *count);
                        found = true;
                    }
                }
                if !found {
                    add_pair(*first_character, *second_character, &mut new_pair_count, *count);
                }
            }
        }
        pair_count = new_pair_count;
    }
    // compute the score
    let mut frequency_count : HashMap<char, usize> = HashMap::new();
    for (first_character, hash_map) in pair_count {
        for (second_character, count) in hash_map {
            if first_character != '0' {
                *frequency_count.entry(first_character).or_insert(0) += count;
            }
            if second_character != '0' {
                *frequency_count.entry(second_character).or_insert(0) += count;
            }
        }
    }
    let mut most_frequent : usize = 0;
    let mut least_frequent : usize = std::usize::MAX;
    for (_char, frequency) in &frequency_count {
        if *frequency > most_frequent {
            most_frequent = *frequency;
        }
        if *frequency < least_frequent {
            least_frequent = *frequency;
        }
    }
    println!("{}", (most_frequent / 2) - (least_frequent / 2));
}

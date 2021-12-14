use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut polymer : String = String::new();
    let mut transformation_rules : HashMap<char, HashMap<char, char>> = HashMap::new();
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
    for _step in 0..2 {
        let mut new_polymer : String = String::new();
        let polymer_chars : Vec<char> = polymer.chars().collect();
        for i in 0..polymer_chars.len() - 1 {
            new_polymer.push(polymer_chars[i]);
            if !transformation_rules.contains_key(&polymer_chars[i]) {
                continue;
            }
            let val = &transformation_rules[&polymer_chars[i]];
            if !val.contains_key(&polymer_chars[i + 1]) {
                continue;
            }
            new_polymer.push(val[&polymer_chars[i + 1]]);
        }
        new_polymer.push(polymer_chars[polymer_chars.len() - 1]);

        polymer = new_polymer;
    }
    // compute the score
    let mut frequency_count : HashMap<char, usize> = HashMap::new();
    for character in polymer.chars() {
        *frequency_count.entry(character).or_insert(1) += 1;
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
    println!("{}", most_frequent - least_frequent);

}

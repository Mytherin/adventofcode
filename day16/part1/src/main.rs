use std::fs::File;
use std::io::{self, BufRead};

// 0..3 bits: packet version
// 3..6 bits: packet type ID
// type id 4: literal value

// each group has 5 bits
// 0 bit: is continuation bit
// 1..5 bits: the number

fn parse_number(position : &mut usize, length : usize, binary_string : &Vec<bool>) -> usize {
    let mut result : usize = 0;
    for i in 0..length {
        result <<= 1;
        if binary_string[*position + i] {
            result += 1;
        }
    }
    *position += length;
    return result;
}

fn parse_literal(position : &mut usize, binary_string : &Vec<bool>) -> usize {
    let mut number : usize = 0;
    loop {
        // check continuation bit
        let continuation_bit = binary_string[*position];
        *position += 1;
        // parse the actual number
        number <<= 4;
        number += parse_number(position, 4, binary_string);
        // increment the position and continue
        if !continuation_bit {
            break;
        }
    }
    return number;
}

fn parse_packet(position : &mut usize, binary_string : &Vec<bool>) -> usize {
    let mut total_version = 0;
    let packet_version = parse_number(position, 3, binary_string);
    let packet_type = parse_number(position, 3, binary_string);
    total_version += packet_version;
    if packet_type == 4 {
        // literal value
        let literal = parse_literal(position, binary_string);
        // println!("ver: {}, type: {}, literal {}", packet_version, packet_type, literal);
    } else {
        // operator
        let length_type_id = binary_string[*position];
        *position += 1;
        let sub_length : usize;
        if !length_type_id {
            // the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
            sub_length = parse_number(position, 15, binary_string);
        } else {
            // the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
            sub_length = parse_number(position, 11, binary_string);
        }
        // println!("ver: {}, type: {}, length type id {}, sub length {}", packet_version, packet_type, length_type_id, sub_length);
        let end_position = *position + sub_length;
        while *position < end_position {
            total_version += parse_packet(position, binary_string)
        }
    }
    return total_version;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    // parse
    let mut hex_string : String = String::new();
    for line in lines {
        hex_string = line.unwrap();
    }
    // convert hex -> binary
    let mut binary_string : String = String::new();

    let mut binary : Vec<bool> = Vec::new();
    for character in hex_string.chars() {
        let new_char = match character {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Unrecognized sequence!")
        };
        for character in new_char.chars() {
            if character == '1' {
                binary_string.push('1');
                binary.push(true);
            } else {
                binary_string.push('0');
                binary.push(false);
            }
        }
    }
    // println!("{}", binary_string);
    let mut position = 0;
    let total_version = parse_packet(&mut position, &binary);
    println!("{}", total_version);
}

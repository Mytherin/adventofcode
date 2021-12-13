use std::fs::File;
use std::io::{self, BufRead};

struct Node {
    name : String,
    neighbors : Vec<usize>,
    is_small : bool,
}

fn find_node(name : &String, nodes : &Vec<Node>) -> usize {
    for i in 0..nodes.len() {
        if nodes[i].name == *name {
            return i;
        }
    }
    return nodes.len();
}

fn add_node(name : String, nodes : &mut Vec<Node>) -> usize {
    let index = find_node(&name, nodes);
    if index < nodes.len() {
        return index;
    }
    let upper = name.to_ascii_uppercase();
    let is_small = upper != name;
    nodes.push(Node { name : name, neighbors : Vec::new(), is_small : is_small});
    return nodes.len() - 1;
}

fn traverse_cave(current_pos : usize, goal : usize, mut visited : Vec<bool>, nodes : &Vec<Node>) -> usize {
    if current_pos == goal {
        return 1;
    }
    if nodes[current_pos].is_small {
        visited[current_pos] = true;
    }
    let mut paths = 0;
    for neighbor in &nodes[current_pos].neighbors {
        if visited[*neighbor] {
            // already visited on this path
            continue;
        }
        paths += traverse_cave(*neighbor, goal, visited.clone(), nodes);
    }
    return paths;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut nodes : Vec<Node> = Vec::new();
    // parse
    for line in lines {
        let unwrapped = line.unwrap();
        let splits : Vec<&str> = unwrapped.split('-').collect();
        let left_index = add_node(splits[0].to_string(), &mut nodes);
        let right_index = add_node(splits[1].to_string(), &mut nodes);
        nodes[left_index].neighbors.push(right_index);
        nodes[right_index].neighbors.push(left_index);
    }
    // where is the start and where is the end
    let start_name : String = "start".to_string();
    let end_name : String = "end".to_string();
    let start_index = find_node(&start_name, &nodes);
    let end_index = find_node(&end_name, &nodes);

    // perform the traversal
    let mut visited : Vec<bool> = Vec::new();
    for _n in 0..nodes.len() {
        visited.push(false);
    }
    let path_count = traverse_cave(start_index, end_index, visited, &nodes);
    println!("{}", path_count);
}

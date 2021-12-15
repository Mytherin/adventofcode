use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node {
    risk : usize,
    lowest_score : usize,
}

#[derive(Hash, Copy, Clone, Eq, PartialEq)]
struct PathNode {
    x_pos : usize,
    y_pos : usize,
    cost : usize
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.x_pos.cmp(&other.x_pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_neighbors(width : usize, height : usize, node : &PathNode) -> Vec<PathNode> {
    let mut neighbors : Vec<PathNode> = Vec::new();
    if node.x_pos > 0 {
        neighbors.push(PathNode { x_pos: node.x_pos - 1, y_pos: node.y_pos, cost: 0 });
    }
    if node.y_pos > 0 {
        neighbors.push(PathNode { x_pos: node.x_pos, y_pos: node.y_pos - 1, cost: 0 });
    }
    if node.x_pos + 1 < width {
        neighbors.push(PathNode { x_pos: node.x_pos + 1, y_pos: node.y_pos, cost: 0 });
    }
    if node.y_pos + 1 < height {
        neighbors.push(PathNode { x_pos: node.x_pos, y_pos: node.y_pos + 1, cost: 0 });
    }
    return neighbors;
}

fn is_goal(width : usize, height : usize, node : &PathNode) -> bool {
    return node.x_pos + 1 == width && node.y_pos + 1 == height;
}

fn find_lowest_cost_path(width : usize, height : usize, map : &mut Vec<Vec<Node>>) -> usize {
    let mut pq = BinaryHeap::new();
    pq.push(PathNode { x_pos: 0, y_pos: 0, cost: 0 });
    while !pq.is_empty() {
        let node : PathNode;
        {
            let entry = pq.pop().unwrap();
            node = entry;
        }
        // println!("{}.{}: {}", node.x_pos, node.y_pos, total_risk);
        if node.cost > map[node.y_pos][node.x_pos].lowest_score {
            continue;
        }
        map[node.y_pos][node.x_pos].lowest_score = node.cost;
        if is_goal(width, height, &node) {
            return node.cost as usize;
        }

        let neighbors = find_neighbors(width, height, &node);
        for mut neighbor in neighbors {
            neighbor.cost = node.cost + map[neighbor.y_pos][neighbor.x_pos].risk;
            // println!("{}.{}: {} + {} = {}", neighbor.x_pos, neighbor.y_pos, total_risk, map[neighbor.y_pos][neighbor.x_pos].risk, new_risk);
            pq.push(neighbor);
        }
    }
    panic!("No path found");
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    let mut map : Vec<Vec<Node>> = Vec::new();
    // parse
    for line in lines {
        let mut hline : Vec<Node> = Vec::new();
        let unwrapped = line.unwrap();
        for character in unwrapped.chars() {
            let risk = character.to_string().parse().unwrap();
            hline.push(Node { risk : risk, lowest_score : std::usize::MAX });
        }
        map.push(hline);
    }
    let width : usize = map[0].len();
    let height : usize = map.len();
    let lowest_cost = find_lowest_cost_path(width, height, &mut map);
    println!("{}", lowest_cost);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::HashMap;

const WIDTH : usize = 14;
const HEIGHT : usize = 7;
const ROOM_START_X : usize = 3;
const ROOM_START_Y : usize = 2;
const ROOM_HEIGHT : usize = 4;
const ROOM_OFFSET_X : usize = 2;
const ROOM_COUNT : usize = 4;
const HALLWAY_Y : usize = 1;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Square {
    Empty,
    Wall,
    Amber,
    Bronze,
    Copper,
    Desert,
    OutOfBounds
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct StateArray {
    array : [Square; WIDTH * HEIGHT]
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct State {
    cost : usize,
    minimum_cost : usize,
    array : StateArray
    // ,history : Vec<State>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x : usize,
    y : usize
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Path {
    cost : usize,
    point : Point
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.minimum_cost.cmp(&self.minimum_cost);
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_index(x : usize, y : usize) -> usize {
    return y * WIDTH + x;
}

fn get_square(x : usize, y : usize, character : char) -> Square {
    let result : Square;
    result = match character {
        '#' => Square::Wall,
        '.' => Square::Empty,
        ' ' => Square::OutOfBounds,
        'A' => Square::Amber,
        'B' => Square::Bronze,
        'C' => Square::Copper,
        'D' => Square::Desert,
        _ => panic!("unrecognized character {}", character)
    };
    return result;
}

fn get_character(square : Square) -> char {
    let result = match square {
        Square::Wall => '#',
        Square::Empty => '.',
        Square::OutOfBounds => ' ',
        Square::Amber => 'A',
        Square::Bronze => 'B',
        Square::Copper => 'C',
        Square::Desert => 'D',
        _ => panic!("unrecognized square")
    };
    return result;
}

fn is_dinosaur(state : Square) -> bool {
    return match state {
    Square::Amber => true,
    Square::Bronze => true,
    Square::Copper => true,
    Square::Desert => true,
    _ => false
    };
}

fn get_regular_dinosaur(state : Square) -> Square {
    return match state {
    Square::Amber => Square::Amber,
    Square::Bronze => Square::Bronze,
    Square::Copper => Square::Copper,
    Square::Desert => Square::Desert,
    _ => panic!("not a start dinosaur")
    };
}

fn dinosaur_move_cost(state : Square) -> usize {
    return match state {
    Square::Amber => 1,
    Square::Bronze => 10,
    Square::Copper => 100,
    Square::Desert => 1000,
    _ => panic!("not a dinosaur")
    };
}

// Amphipods will never stop on the space immediately outside any room. They can move into that space so long as they immediately continue moving. (Specifically, this refers to the four open spaces in the hallway that are directly above an amphipod starting position.)

// Amphipods will never move from the hallway into a room unless that room is their destination room and that room contains no amphipods which do not also have that room as their own destination. If an amphipod's starting room is not its destination room, it can stay in that room until it leaves the room. (For example, an Amber amphipod will not move from the hallway into the right three rooms, and will only move into the leftmost room if that room is empty or if it only contains other Amber amphipods.)

// Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room. (That is, once any amphipod starts moving, any other amphipods currently in the hallway are locked in place and will not move again until they can move fully into a room.)

fn is_hallway(x : usize, y : usize) -> bool {
    return y == HALLWAY_Y;
}

fn is_room(x : usize, y : usize) -> bool {
    return !is_hallway(x, y);
}

fn is_outside_room(x : usize, y : usize) -> bool {
    return is_hallway(x, y) && (x == 3 || x == 5 || x == 7 || x == 9);
}

fn is_amber(s : Square) -> bool {
    return s == Square::Amber;
}

fn is_bronze(s : Square) -> bool {
    return s == Square::Bronze;
}

fn is_copper(s : Square) -> bool {
    return s == Square::Copper;
}

fn is_desert(s : Square) -> bool {
    return s == Square::Desert;
}

fn correct_x_position(s : Square) -> usize {
    if is_amber(s) {
        return ROOM_START_X;
    }
    if is_bronze(s) {
        return ROOM_START_X + ROOM_OFFSET_X;
    }
    if is_copper(s) {
        return ROOM_START_X + ROOM_OFFSET_X * 2;
    }
    if is_desert(s) {
        return ROOM_START_X + ROOM_OFFSET_X * 3;
    }
    panic!("not a dinosaur");
}

fn is_correct_room(s : Square, x : usize, y : usize) -> bool {
    if !is_dinosaur(s) {
        return false;
    }
    return x == correct_x_position(s);
}


fn is_matching_dinosaur(a : Square, b : Square) -> bool {
    if is_amber(a) && is_amber(b) {
        return true;
    }
    if is_bronze(a) && is_bronze(b) {
        return true;
    }
    if is_copper(a) && is_copper(b) {
        return true;
    }
    if is_desert(a) && is_desert(b) {
        return true;
    }
    return false;
}

impl State {
    fn get_state(self : &State, x : usize, y : usize) -> Square {
        return self.array.array[get_index(x, y)];
    }
    fn set_state(self : &mut State, x : usize, y : usize, state : Square) {
        self.array.array[get_index(x, y)] = state;
    }
    fn print(self : &State) {
        // for history in &self.history {
        //     history.print();
        // }
        println!("------ STATE COST {} ----- ", self.cost);
        for y in 0..HEIGHT {
            let mut str : String = String::new();
            for x in 0..WIDTH {
                str.push(get_character(self.get_state(x, y)));
            }
            println!("{}", str);
        }
    }
    fn is_finished(self : &State) -> bool {
        let mut x : usize = ROOM_START_X;
        let mut y : usize = ROOM_START_Y;
        for _i in 0..ROOM_COUNT {
            for offset in 0..ROOM_HEIGHT {
                let dino = self.get_state(x, y + offset);
                if !is_dinosaur(dino) {
                    return false;
                }
                if !is_correct_room(dino, x, y + offset) {
                    return false;
                }
            }
            x += ROOM_OFFSET_X;
        }
        return true;
    }
    fn is_walkable(self : &State, x : usize, y : usize) -> bool {
        return self.get_state(x, y) == Square::Empty;
    }
    fn find_neighbors(self : &State, path : Path, moving_cost : usize) -> Vec<Path> {
        let mut result : Vec<Path> = Vec::new();
        let p = path.point;
        if self.is_walkable(p.x - 1, p.y) {
            result.push(Path { cost: path.cost + moving_cost, point: Point { x: p.x - 1, y: p.y }});
        }
        if self.is_walkable(p.x + 1, p.y) {
            result.push(Path { cost: path.cost + moving_cost, point: Point { x: p.x + 1, y: p.y }});
        }
        if self.is_walkable(p.x, p.y - 1) {
            result.push(Path { cost: path.cost + moving_cost, point: Point { x: p.x, y: p.y - 1}});
        }
        if self.is_walkable(p.x, p.y + 1) {
            result.push(Path { cost: path.cost + moving_cost, point: Point { x: p.x, y: p.y + 1 }});
        }
        return result;
    }
    fn possible_solution(self : &State, moving_dinosaur : Square, x : usize, y : usize) -> bool {
        if !is_room(x, y) {
            panic!("this should be a room");
        }
        if !is_correct_room(moving_dinosaur, x, y) {
            return false;
        }
        // we should move to the lowest room possible only
        // any other move does not make sense
        for y_offset in 0..ROOM_HEIGHT {
            let current_y = 1 + ROOM_HEIGHT - y_offset;
            let other_state = self.get_state(x, current_y);
            if is_matching_dinosaur(moving_dinosaur, other_state) {
                continue;
            }
            if is_dinosaur(other_state) {
                return false;
            }
            return y == current_y;
        }
        return false;
    }

    fn get_minimal_cost(self : &State) -> usize {
        // any dinosaur in the hallway will need to move down at least one step
        let mut minimum_cost : usize = self.cost;
        for x in 0..WIDTH {
            let hall_state = self.get_state(x, HALLWAY_Y);
            if is_dinosaur(hall_state) {
                minimum_cost += dinosaur_move_cost(hall_state);
            }
        }
        // any dinosaur that is not in the correct X position will need to move to the correct X position
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let dino = self.get_state(x, y);
                if is_dinosaur(dino) {
                    minimum_cost += ((x as i64) - (correct_x_position(dino) as i64)).abs() as usize;
                }
            }
        }
        return minimum_cost;
    }

    fn find_paths(self : &State, x : usize, y : usize, result : &mut Vec<State>, start_from_room : bool) {
        // dinosaurs in a hallway can only move to a room
        // do a BFS to find all possible locations
        let moving_dinosaur = self.get_state(x, y);
        // if start_from_room && !is_start_dinosaur(moving_dinosaur) {
        //     panic!("eek");
        // }
        let next_dinosaur = get_regular_dinosaur(moving_dinosaur);
        let moving_cost = dinosaur_move_cost(moving_dinosaur);
        let mut visited_points : HashSet<Point> = HashSet::new();
        let mut open_paths : Vec<Path> = Vec::new();
        open_paths.push(Path { cost: 0, point : Point { x: x, y: y }});
        visited_points.insert(open_paths[0].point);
        let mut path_index = 0;
        while path_index < open_paths.len() {
            let path = open_paths[path_index];
            let neighbors = self.find_neighbors(path, moving_cost);
            for neighbor in neighbors {
                if visited_points.contains(&neighbor.point) {
                    continue;
                }
                // we can only move into the hallway if we start from a room
                let mut is_movable_hallway = start_from_room && is_hallway(neighbor.point.x, neighbor.point.y) && !is_outside_room(neighbor.point.x, neighbor.point.y);
                // we can always move into a room
                let mut is_movable_room = is_room(neighbor.point.x, neighbor.point.y) && self.possible_solution(moving_dinosaur, neighbor.point.x, neighbor.point.y);
                if start_from_room && is_movable_room && neighbor.point.x == x && neighbor.point.y < y {
                    is_movable_room = false;
                }
                if is_movable_hallway || is_movable_room {
                    let mut new_state : State = self.clone();
                    new_state.set_state(x, y, Square::Empty);
                    new_state.set_state(neighbor.point.x, neighbor.point.y, next_dinosaur);
                    new_state.cost += neighbor.cost;
                    new_state.minimum_cost = new_state.get_minimal_cost();
                    // new_state.history.push(self.clone());
                    result.push(new_state);
                }
                visited_points.insert(neighbor.point);
                open_paths.push(neighbor);
            }
            path_index += 1;
        }
    }

    fn get_state_transitions(self : &State) -> Vec<State> {
        let mut result : Vec<State> = Vec::new();
        // figure out the dinosaurs that can move
        // we can either move dinosaurs in the hallway
        for x in 0..WIDTH {
            if is_dinosaur(self.get_state(x, HALLWAY_Y)) {
                // we can move dinosaurs in the hallway
                // generate moves for this dinosaur
                self.find_paths(x, HALLWAY_Y, &mut result, false);
            }
        }
        // OR dinosaurs in a room that haven't moved yet
        for x in 0..ROOM_COUNT {
            for y in 0..ROOM_HEIGHT {
                let dino_x : usize = ROOM_START_X + x * 2;
                let dino_y : usize = ROOM_START_Y + y;
                let dino = self.get_state(dino_x, dino_y);
                if is_dinosaur(dino) {
                    self.find_paths(dino_x, dino_y, &mut result, true);
                }
            }
        }
        return result;
    }
}

fn find_lowest_cost_path(initial_state : State) -> usize {
    let mut pq = BinaryHeap::new();
    pq.push(initial_state.clone());
    let mut seen_states : HashMap<StateArray, usize> = HashMap::new();
    seen_states.insert(initial_state.array.clone(), 0);
    while !pq.is_empty() {
        let node : State;
        {
            let entry = pq.pop().unwrap();
            node = entry;
        }
        // println!("cost {}, node count {}", node.cost, pq.len());
        if node.is_finished() {
            node.print();
            return node.cost as usize;
        }
        if seen_states[&node.array] < node.cost {
            continue;
        }

        let neighbors = node.get_state_transitions();
        for mut neighbor in neighbors {
            if seen_states.contains_key(&neighbor.array) {
                let cost = seen_states[&neighbor.array];
                if neighbor.cost >= cost {
                    continue;
                } else {
                    *seen_states.get_mut(&neighbor.array).unwrap() = neighbor.cost;
                }
            } else {
                seen_states.insert(neighbor.array.clone(), neighbor.cost);
            }
            pq.push(neighbor);
        }
    }
    panic!("No path found");
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut initial_state : State = State { cost: 0,
        minimum_cost: 0,
        array: StateArray { array: [Square::OutOfBounds; WIDTH * HEIGHT]}
    // , history : Vec::new()
};
    initial_state.minimum_cost = initial_state.get_minimal_cost();
    let mut y = 0;
    for line in buf_lines {
        let unwrapped_line = line.unwrap();
        let mut x = 0;
        for character in unwrapped_line.chars() {
            initial_state.set_state(x, y, get_square(x, y, character));
            x += 1;
        }
        y += 1;
    }
    initial_state.print();

    let cost = find_lowest_cost_path(initial_state);
    println!("{}", cost);
}

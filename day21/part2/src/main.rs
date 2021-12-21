const PLAYER_ONE_START : usize = 10;
const PLAYER_TWO_START : usize = 6;
use std::collections::HashMap;

// const PLAYER_ONE_START : usize = 4;
// const PLAYER_TWO_START : usize = 8;
const WINNING_SCORE : usize = 21;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Players {
    player_turn : usize,
    position_one : usize,
    score_one : usize,
    position_two : usize,
    score_two : usize,
}

struct ScoreBoard {
    player_one_victories : usize,
    player_two_victories : usize
}

// 3: 1
// 4: 3
// 5: 6
// 6: 7
// 7: 6
// 8: 3
// 9: 1

fn perform_roll(
    current : Players,
    universe_count : usize,
    scores : &mut ScoreBoard,
    universes : &mut HashMap<Players, usize>,
    open_configurations : &mut Vec<Players>) {
    let roll_values = vec![3, 4, 5, 6, 7, 8, 9];
    let roll_entries = vec![1, 3, 6, 7, 6, 3, 1];
    // this duplication is really ugly but whatever
    if current.player_turn == 0 {
        for i in 0..roll_values.len() {
            let position = (current.position_one + roll_values[i]) % 10;
            let score = current.score_one + position + 1;
            let new_count = universe_count * roll_entries[i];
            if score >= WINNING_SCORE {
                scores.player_one_victories += new_count;
            } else {
                let result = Players {
                    player_turn: 1,
                    position_one: position,
                    score_one: score,
                    position_two: current.position_two,
                    score_two: current.score_two,
                };
                open_configurations.push(result.clone());
                *universes.entry(result).or_insert(0) += new_count;
            }
        }
    } else {
        for i in 0..roll_values.len() {
            let position = (current.position_two + roll_values[i]) % 10;
            let score = current.score_two + position + 1;
            let new_count = universe_count * roll_entries[i];
            if score >= WINNING_SCORE {
                scores.player_two_victories += new_count;
            } else {
                let result = Players {
                    player_turn: 0,
                    position_one: current.position_one,
                    score_one: current.score_one,
                    position_two: position,
                    score_two: score,
                };
                open_configurations.push(result.clone());
                *universes.entry(result).or_insert(0) += new_count;
            }
        }

    }

}

fn print_universes(universes : &HashMap<Players, usize>) {
    for (state, count) in universes {
        println!("turn: {} p1: {}, s1: {}, p2: {}, s2: {}, count: {}", state.player_turn, state.position_one+1, state.score_one, state.position_two+1, state.score_two, count);
    }
}

fn main() {
    let mut universes : HashMap<Players, usize> = HashMap::new();
    let mut open_configurations : Vec<Players> = Vec::new();
    let mut score_board : ScoreBoard = ScoreBoard {
        player_one_victories: 0,
        player_two_victories: 0
    };
    let start_configuration : Players = Players {
        player_turn: 0,
        position_one: PLAYER_ONE_START - 1,
        score_one: 0,
        position_two: PLAYER_TWO_START - 1,
        score_two: 0
    };
    universes.entry(start_configuration).or_insert(1);
    open_configurations.push(start_configuration);
    let mut configuration_index = 0;
    while configuration_index < open_configurations.len() {
        let config = open_configurations[configuration_index];
        let mut universe_count : usize = 0;
        match universes.get(&config) {
            Some(c) => {
                universe_count = *c;
                if universe_count <= 0 {
                    panic!("eek");
                }
            },
            None => (),
        };
        if universe_count > 0 {
            universes.remove(&config);
            perform_roll(config, universe_count, &mut score_board, &mut universes, &mut open_configurations);
        }
        configuration_index += 1;
    }
    println!("{}", score_board.player_one_victories);
    println!("{}", score_board.player_two_victories);
}

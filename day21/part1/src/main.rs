const PLAYER_ONE_START : usize = 10;
const PLAYER_TWO_START : usize = 6;

// const PLAYER_ONE_START : usize = 4;
// const PLAYER_TWO_START : usize = 8;


// Play a practice game using the deterministic 100-sided die. The moment either player wins, what do you get if you multiply the score of the losing player by the number of times the die was rolled during the game?

struct Player {
    index : usize,
    position : usize,
    score : usize
}

fn roll_die(current : &mut usize) -> usize {
    let mut result : usize = 0;
    for i in 0..3 {
        result += *current;
        *current += 1;
    }
    return result;
}



fn main() {
    let mut players : Vec<Player> = Vec::new();
    players.push(Player { index: 0 , position: PLAYER_ONE_START - 1, score: 0});
    players.push(Player { index: 1 , position: PLAYER_TWO_START - 1, score: 0});

    let mut deterministic_die : usize = 1;
    let mut finished : bool = false;
    let mut total_rolls = 0;
    loop {
        for player in &mut players {
            let roll = roll_die(&mut deterministic_die);
            total_rolls += 3;
            player.position = (player.position + roll) % 10;
            player.score += player.position + 1;
            if player.score >= 1000 {
                finished = true;
                break;
            }
        }
        if finished {
            break;
        }
    }
    let mut losing_score : usize = 0;
    for player in &players {
        if player.score < 1000 {
            losing_score = player.score;
        }
    }
    println!("{}", losing_score * total_rolls);
}

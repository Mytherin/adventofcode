use std::fs::File;
use std::io::{self, BufRead};

const BOARD_SIZE : usize = 5;

struct BingoPoint {
    number : i32,
    marked : bool
}

impl BingoPoint {
    fn new(number_p : i32) -> BingoPoint {
        return BingoPoint { number: number_p, marked: false };
    }
}

struct BingoBoard {
    board : Vec<Vec<BingoPoint>>,
    won : bool
}

impl BingoBoard {
    fn new() -> BingoBoard {
        return BingoBoard { board: Vec::new(), won: false };
    }

    fn Check(&self) -> bool {
        // check for a horizontal match
        for y in 0..BOARD_SIZE {
            let mut all_marked = true;
            for x in 0..BOARD_SIZE {
                if !self.board[x][y].marked {
                    all_marked = false;
                }
            }
            if all_marked {
                return true;
            }
        }
        // check for a vertical match
        for x in 0..BOARD_SIZE {
            let mut all_marked = true;
            for y in 0..BOARD_SIZE {
                if !self.board[x][y].marked {
                    all_marked = false;
                }
            }
            if all_marked {
                return true;
            }
        }
        return false;
    }

    fn MarkRow(&mut self, number : i32) -> bool {
        if self.won {
            // board already won
            return false;
        }
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[x][y].number == number {
                    self.board[x][y].marked = true;
                }
            }
        }
        if self.Check() {
            self.won = true;
            return true;
        }
        return false;
    }

    fn Score(&self, last_number : i32) -> i32 {
        let mut sum : i32 = 0;
        for row in &self.board {
            for point in row {
                if !point.marked {
                    sum += point.number;
                }
            }
        }
        return sum * last_number;
    }
}

fn parse_number(line : &str, offset : usize) -> i32 {
    if &line[offset..offset+1] == " " {
        return line[offset + 1..offset + 2].parse::<i32>().unwrap();
    } else {
        return line[offset..offset + 2].parse::<i32>().unwrap();
    }
}


fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut lines : Vec<String> = Vec::new();
    for line in buf_lines {
        lines.push(line.unwrap());
    }

    let mut numbers: Vec<i32> = Vec::new();
    let mut boards: Vec<BingoBoard> = Vec::new();
    let split_numbers = lines[0].split(",");
    for number in split_numbers {
        numbers.push(number.parse().unwrap())
    }
    let mut current_board : BingoBoard = BingoBoard::new();
    for i in 2..lines.len() {
        if lines[i].len() == 0 {
            if current_board.board.len() > 0 {
                if current_board.board.len() != BOARD_SIZE {
                    panic!("Illegally sized board!")
                }
                boards.push(current_board);
                current_board = BingoBoard::new();
            }
            continue;
        }
        let mut current_board_line : Vec<BingoPoint> = Vec::new();
        current_board_line.push(BingoPoint::new(parse_number(&lines[i], 0)));
        current_board_line.push(BingoPoint::new(parse_number(&lines[i], 3)));
        current_board_line.push(BingoPoint::new(parse_number(&lines[i], 6)));
        current_board_line.push(BingoPoint::new(parse_number(&lines[i], 9)));
        current_board_line.push(BingoPoint::new(parse_number(&lines[i], 12)));
        current_board.board.push(current_board_line);
    }
    if current_board.board.len() > 0 {
        if current_board.board.len() != BOARD_SIZE {
            panic!("Illegally sized board!")
        }
        boards.push(current_board);
    }
    let mut total_winning_boards : usize = 0;
    let total_boards = boards.len();
    for number in numbers {
        for board in &mut boards {
            if board.MarkRow(number) {
                total_winning_boards += 1;
                if total_winning_boards == total_boards {
                    println!("{}", board.Score(number));
                    return;
                }
            }
        }
    }
}

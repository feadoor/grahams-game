mod board;
mod evaluator;
mod movegen;
mod negamax;

use board::Board;
use evaluator::{evaluate, EvaluationResult};
use movegen::{generate_moves, Move};
use negamax::best_moves;

use rand::prelude::*;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut board = Board::new();
    loop {
        println!("\n{}", &board); stdout().flush().unwrap();

        if let EvaluationResult::Terminal(score) = evaluate(&board) {
            match score {
                0 => println!("Game over - it's a draw!"),
                _ => match is_players_turn(&board) {
                    true => println!("Game over - I win!"),
                    false => println!("Game over - you beat me... :("),
                },
            }
            break; 
        }

        if is_players_turn(&board) {
            let m = get_move_from_user(&board);
            board.place_digit(m.position, m.digit);
        } else {
            let moves = best_moves(&mut board);
            let m = thread_rng().choose(&moves).unwrap();

            board.place_digit(m.mov.position, m.mov.digit);
            match m.score.signum() {
                1 => println!("I'm going to win, and there's nothing you can do about it!"),
                0 => println!("You can't beat me now"),
                -1 => println!("Looks like you're getting the better of me..."),
                _ => unreachable!(),
            }
        }
    }
}

fn is_players_turn(board: &Board) -> bool {
    board.count_placed_digits() % 2 == 0
}

fn get_move_from_user(board: &Board) -> Move {
    print!("Enter your move > "); stdout().flush().unwrap();
    let mut s = String::new(); stdin().read_line(&mut s).unwrap();

    if s.len() >= 2 {
        let bytes = s.as_bytes();
        if bytes[0] >= b'A' && bytes[0] <= b'I' && bytes[1] >= b'1' && bytes[1] <= b'9' {
            let m = Move { position: (bytes[0] - b'A') as usize, digit: bytes[1] - b'0' };
            if generate_moves(&board).contains(&m) { return m; }
        }
    }

    println!("Didn't understand your move"); stdout().flush().unwrap();
    get_move_from_user(&board)
}

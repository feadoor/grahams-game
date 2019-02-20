use crate::board::Board;
use crate::evaluator::{evaluate, EvaluationResult, MAX_SCORE};
use crate::movegen::{generate_moves, Move};

pub struct NegamaxMove{
    pub mov: Move, 
    pub score: i32,
}

pub fn best_moves(board: &mut Board) -> Vec<NegamaxMove> {
    let moves = generate_moves(&board);
    let mut best_score = -MAX_SCORE; let mut best_moves = Vec::new();

    for m in &moves {
        board.place_digit(m.position, m.digit);
        let score = -alpha_beta(board, -MAX_SCORE, MAX_SCORE);
        board.remove_digit(m.position);

        if score > best_score {
            best_score = score;
            best_moves.clear();
        }

        if score == best_score {
            best_moves.push(NegamaxMove { mov: m.clone(), score: best_score });
        }
    }

    best_moves
}

fn alpha_beta(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {

    if let EvaluationResult::Terminal(score) = evaluate(&board) {
        return score;
    }

    let moves = generate_moves(&board);
    for m in &moves {
        board.place_digit(m.position, m.digit);
        let score = -alpha_beta(board, -beta, -alpha);
        board.remove_digit(m.position);

        if score >= alpha { alpha = score; }
        if alpha >= beta { break; }
    }

    alpha
}

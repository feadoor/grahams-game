use crate::board::Board;

pub const MAX_SCORE: i32 = 1000;
const LINE_POSITIONS: &'static [[usize; 3]; 8] = &[
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

pub enum EvaluationResult {
    Terminal(i32),
    NonTerminal,
}

pub fn evaluate(board: &Board) -> EvaluationResult {

    for &line in LINE_POSITIONS {
        if line.iter().all(|&p| board.get_digit_at(p) != 0) {
            if line.iter().map(|&p| board.get_digit_at(p)).sum::<u8>() == 15 {
                let raw_score = MAX_SCORE / (board.count_placed_digits() as i32);
                return EvaluationResult::Terminal(-raw_score);
            }
        }
    }

    if board.count_empty_positions() == 0 {
        EvaluationResult::Terminal(0)
    } else {
        EvaluationResult::NonTerminal
    }
}

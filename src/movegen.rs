use crate::board::Board;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub position: usize,
    pub digit: u8,
}

pub fn generate_moves(board: &Board) -> Vec<Move> {

    let allowed_digits = board.get_unused_digits(digits_for_next_turn(&board));
    let allowed_positions = board.get_empty_positions();

    let mut moves = Vec::new();
    for &digit in &allowed_digits {
        for &position in &allowed_positions {
            moves.push(Move { position, digit });
        }
    }
    moves
}

fn digits_for_next_turn(board: &Board) -> &'static [u8] {
    static PLAYER_1: [u8; 5] = [1, 3, 5, 7, 9];
    static PLAYER_2: [u8; 4] = [2, 4, 6, 8];

    match board.count_placed_digits() % 2 {
        0 => &PLAYER_1,
        1 => &PLAYER_2,
        _ => unreachable!(),
    }
}
use std::char;
use std::fmt;

pub struct Board {
    cells: Vec<u8>,
    used_digits: u16,
    bit_representation: u64,
}

impl Board {

    pub fn new() -> Board {
        Board { cells: vec![0; 9], used_digits: 0, bit_representation: 0 }
    }

    pub fn place_digit(&mut self, position: usize, digit: u8) {
        self.cells[position] = digit;
        self.used_digits |= 1 << digit;
        self.bit_representation |= (digit as u64) << (4 * position);
    }

    pub fn remove_digit(&mut self, position: usize) {
        self.bit_representation &= !(15 << (4 * position));
        self.used_digits &= !(1 << self.cells[position]);
        self.cells[position] = 0;
    }

    pub fn get_unused_digits(&self, potential_digits: &[u8]) -> Vec<u8> {
        potential_digits.iter().filter(|&&d| self.digit_is_available(d)).map(|x| *x).collect()
    }

    pub fn get_empty_positions(&self) -> Vec<usize> {
        (0..9).filter(|&p| self.position_is_empty(p)).collect()
    }

    pub fn get_digit_at(&self, position: usize) -> u8 {
        self.cells[position]
    }

    pub fn count_placed_digits(&self) -> u32 {
        self.used_digits.count_ones()
    }

    pub fn count_empty_positions(&self) -> u32 {
        9 - self.count_placed_digits()
    }

    fn position_is_empty(&self, position: usize) -> bool {
        self.cells[position] == 0
    }

    fn digit_is_available(&self, digit: u8) -> bool {
        self.used_digits & (1 << digit) == 0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " {} | {} | {} ", digit_char(self.cells[0], 'A'), digit_char(self.cells[1], 'B'), digit_char(self.cells[2], 'C'))?;
        writeln!(f, "---+---+---")?;
        writeln!(f, " {} | {} | {} ", digit_char(self.cells[3], 'D'), digit_char(self.cells[4], 'E'), digit_char(self.cells[5], 'F'))?;
        writeln!(f, "---+---+---")?;
        writeln!(f, " {} | {} | {} ", digit_char(self.cells[6], 'G'), digit_char(self.cells[7], 'H'), digit_char(self.cells[8], 'I'))
    }
}

fn digit_char(digit: u8, default: char) -> char {
    match digit {
        0 => default,
        _ => char::from_digit(digit.into(), 10).unwrap(),
    }
}
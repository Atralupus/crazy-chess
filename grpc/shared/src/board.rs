use crate::bitboard::{clear_bit, is_bit_set, set_bit, Bitboard};
use crate::mov::Move;

#[derive(Clone)]
pub struct Board {
    pub pawns: Bitboard,
    pub side_to_move: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut pawns = 0;

        for i in 8..16 {
            set_bit(&mut pawns, i);
        }
        for i in 48..56 {
            set_bit(&mut pawns, i);
        }

        Self {
            pawns,
            side_to_move: true,
        }
    }

    pub fn move_pawn(&mut self, mv: Move) -> bool {
        if !is_bit_set(self.pawns, mv.from) {
            return false;
        }

        let rank_from = mv.from / 8;
        let rank_to = mv.to / 8;
        let is_white = self.side_to_move;
        let valid = if is_white {
            (rank_from == 1 && rank_to == 3) || (rank_to == rank_from + 1)
        } else {
            (rank_from == 6 && rank_to == 4) || (rank_to == rank_from - 1)
        };

        if valid {
            clear_bit(&mut self.pawns, mv.from);
            set_bit(&mut self.pawns, mv.to);
            self.side_to_move = !self.side_to_move;
            true
        } else {
            false
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

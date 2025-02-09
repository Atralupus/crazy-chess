pub type Bitboard = u64;

pub fn set_bit(board: &mut Bitboard, square: usize) {
    *board |= 1 << square;
}

pub fn clear_bit(board: &mut Bitboard, square: usize) {
    *board &= !(1 << square);
}

pub fn is_bit_set(board: Bitboard, square: usize) -> bool {
    (board & (1 << square)) != 0
}

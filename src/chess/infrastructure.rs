#[derive(Clone)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Clone)]
pub enum Side {
    BLACK,
    WHITE,
}

impl Side {
    pub fn adjust_pawn_move_offset(&self, row_offset: &i32) -> i32 {
        match self {
            Side::BLACK => { -row_offset.clone() }
            Side::WHITE => { row_offset.clone() }
        }
    }
}


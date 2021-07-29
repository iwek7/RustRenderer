#[derive(Clone)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

pub enum Side {
    BLACK,
    WHITE,
}
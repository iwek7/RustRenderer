use crate::chess::chessboard::ChessboardState;
use crate::chess::field::{Field, FieldLogic};
use crate::opengl_context::OpenglContext;

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
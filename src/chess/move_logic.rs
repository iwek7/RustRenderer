use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::infrastructure::{PieceType, VectorExtension};
use crate::chess::piece::PieceLogic;

pub trait PieceMoveComponent {
    fn is_move_allowed(&self, state: &ChessboardState, target_field: &FieldLogic, piece_to_move: &PieceLogic) -> bool {
        println!("Checking allowed move to {:?}", piece_to_move.get_occupied_field());
        self.get_all_allowed_moves(state, piece_to_move).contains(target_field)
    }
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<FieldLogic>;
}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {
    fn get_all_allowed_moves(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<FieldLogic> {
        let mut allowed_moves = vec!();

        allowed_moves.push_if_exists(self.get_move_ahead(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_first_move(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_left_capture(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_right_capture(chessboard, piece_to_move));



        return allowed_moves;
    }
}

// todo: enpassant
impl PawnMoveComponent {
    fn get_move_ahead(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<FieldLogic> {
        match piece_to_move.get_occupied_field().get_offset_field(0,piece_to_move.get_side().adjust_pawn_move_offset(&1)) {
            None => None,
            Some(field_ahead) => if chessboard.is_field_empty(&field_ahead) {
                Some(field_ahead)
            } else {
                None
            }
        }
    }

    fn get_first_move(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<FieldLogic> {
        if piece_to_move.has_moved() {
            return None;
        }
        match piece_to_move.get_occupied_field().get_offset_field(0,piece_to_move.get_side().adjust_pawn_move_offset(&2)) {
            None => None,
            Some(field_ahead) => if chessboard.is_field_empty(&field_ahead) {
                Some(field_ahead)
            } else {
                None
            }
        }
    }



    fn get_left_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<FieldLogic> {
        self.get_capture(chessboard, piece_to_move, -1)
    }

    fn get_right_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<FieldLogic> {
        self.get_capture(chessboard, piece_to_move, 1)
    }

    fn get_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic, col_offset: i32) -> Option<FieldLogic> {
        match piece_to_move.get_occupied_field().get_offset_field(col_offset, piece_to_move.get_side().adjust_pawn_move_offset(&1)) {
            None => None,
            Some(attacked_field) => {
                let attacked_piece = chessboard.get_piece_at(&attacked_field);
                if attacked_piece.is_some() && attacked_piece.unwrap().get_side() != piece_to_move.get_side() {
                    return Some(attacked_field)
                }
                return None
            }
        }
    }
}

pub struct RockMoveComponent {}

impl PieceMoveComponent for RockMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<FieldLogic> {
        return get_all_fields();
    }
}

pub fn create_move_component(piece_type: &PieceType) -> Box<dyn PieceMoveComponent> {
    match piece_type {
        PieceType::ROOK => Box::new(RockMoveComponent {}),
        _ => Box::new(PawnMoveComponent {})
    }
}

fn get_all_fields() -> Vec<FieldLogic> {
    let mut vec = vec!();
    for i in 1..=8 {
        vec.push(FieldLogic::from_string(format!("A{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("B{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("C{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("D{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("E{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("F{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("G{}", i).as_str()));
        vec.push(FieldLogic::from_string(format!("H{}", i).as_str()));
    }
    return vec;
}


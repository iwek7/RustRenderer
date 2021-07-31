use crate::chess::allowed_move::{AllowedMove, AllowedMoves, MoveType};
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::infrastructure::{PieceType, VectorExtension};
use crate::chess::piece::PieceLogic;

/**
piece move trait
 */
pub trait PieceMoveComponent {
    fn is_move_allowed(&self, state: &ChessboardState, target_field: &FieldLogic, piece_to_move: &PieceLogic) -> Option<AllowedMove> {
        println!("Checking allowed move to {:?}", piece_to_move.get_occupied_field());
        return self.get_all_allowed_moves(state, piece_to_move).get_allowed_move_to(target_field);
    }

    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves;

    fn get_all_attacked_fields(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        self.get_all_allowed_moves(state, piece_to_move)
    }
}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {
    fn get_all_allowed_moves(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut allowed_moves = vec!();
        allowed_moves.push_if_exists(self.get_move_ahead(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_first_move(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_left_capture(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_right_capture(chessboard, piece_to_move));

        return AllowedMoves::new(allowed_moves);
    }

    fn get_all_attacked_fields(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut attacked_fields = vec!();
        attacked_fields.push_if_exists(AllowedMove::attack_to_field(piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), -1));
        attacked_fields.push_if_exists(AllowedMove::attack_to_field(piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), 1));
        return AllowedMoves::new(attacked_fields);
    }
}

/**
pawn move
 */
// todo: enpassant
impl PawnMoveComponent {
    fn get_move_ahead(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedMove> {
        let allowed_move = AllowedMove::move_to_field(chessboard, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), 0);
        return match allowed_move {
            None => { None }
            Some(real_move) => {
                if real_move.get_move_type() == MoveType::MOVE {
                    Some(real_move)
                } else {
                    None
                }
            }
        };
    }

    fn get_first_move(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedMove> {
        if piece_to_move.has_moved() {
            return None;
        }

        // slightly inefficient as we look twice for first move (here and in calling func)
        // on the other hand imo this makes code of parent func clearer
        match self.get_move_ahead(chessboard, piece_to_move) {
            None => { return None; }
            Some(_) => {}
        }

        match piece_to_move.get_occupied_field().get_offset_field(0, piece_to_move.get_side().adjust_pawn_move_offset(&2)) {
            None => None,
            Some(field_ahead) => if chessboard.is_field_empty(&field_ahead) {
                Some(AllowedMove::new_move(field_ahead))
            } else {
                None
            }
        }
    }

    fn get_left_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedMove> {
        self.get_capture(chessboard, piece_to_move, -1)
    }

    fn get_right_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedMove> {
        self.get_capture(chessboard, piece_to_move, 1)
    }

    fn get_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic, col_offset: i32) -> Option<AllowedMove> {
        match AllowedMove::move_to_field(chessboard, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), col_offset) {
            None => { None }
            Some(allowed_field) => {
                if allowed_field.get_move_type() == MoveType::CAPTURE {
                    return Some(allowed_field);
                } else {
                    None
                }
            }
        }
    }
}

/**
rook move
 */
pub struct RockMoveComponent {}

impl PieceMoveComponent for RockMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut moves = vec!();
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 1, 0));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 0, 1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, -1, 0));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 0, -1));
        AllowedMoves::new(moves)
    }
}

/**
bishop move
 */
pub struct BishopMoveComponent {}

impl PieceMoveComponent for BishopMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut moves = vec!();
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 1, 1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 1, -1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, -1, 1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, -1, -1));
        AllowedMoves::new(moves)
    }
}

/**
Knight move
 */
pub struct KnightMoveComponent {}

impl PieceMoveComponent for KnightMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut moves = vec!();
        // todo: write macro that takes all args and adds only not empty optionals

        // move up
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 2, 1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 2, -1));

        // move right
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -1, 2));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 1, 2));

        // move down
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -2, 1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -2, -1));

        // move left
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -1, -2));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 1, -2));

        return AllowedMoves::new(moves);
    }
}

/**
Queen move
 */

pub struct QueenMoveComponent {}

impl PieceMoveComponent for QueenMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut moves = vec!();
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 1, 1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 1, -1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, -1, 1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, -1, -1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 1, 0));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 0, 1));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, -1, 0));
        moves.append(&mut AllowedMove::get_moves_in_direction(state, piece_to_move, 0, -1));
        AllowedMoves::new(moves)
    }
}

/**
King move
 */

pub struct KingMoveComponent {}

impl PieceMoveComponent for KingMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let attacked_fields = state.get_all_attacked_fields(piece_to_move.get_side());
        return AllowedMoves::new(self.get_potential_moves(state, piece_to_move).iter()
            .filter(|allowed_move| !attacked_fields.contains(allowed_move.get_target()))
            .cloned()
            .collect()
        );
    }

    fn get_all_attacked_fields(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        AllowedMoves::new(self.get_potential_moves(state, piece_to_move))
    }
}

impl KingMoveComponent {
    fn get_potential_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedMove> {
        let mut moves = vec!();
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 1, 0));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 1, 1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 0, 1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -1, 1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -1, 0));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, -1, -1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 0, -1));
        moves.push_if_exists(AllowedMove::move_to_field(state, piece_to_move, 1, -1));

        // castles

        // todo implement rule that prevents castle if field is attacked
        // king side castle
        if !piece_to_move.has_moved() {
            let one_right = piece_to_move.get_occupied_field().get_offset_field(1, 0).unwrap();
            let two_right = piece_to_move.get_occupied_field().get_offset_field(2, 0).unwrap();
            let rook_field = piece_to_move.get_occupied_field().get_offset_field(3, 0).unwrap();
            let possible_rook = state.get_piece_at(&rook_field);
            if possible_rook.is_some() {
                let rook = possible_rook.unwrap();
                if !rook.has_moved()
                    && !state.is_field_occupied(&one_right)
                    && !state.is_field_occupied(&two_right) {
                    moves.push_if_exists(Some(AllowedMove::new_composite_move(two_right, one_right, rook.clone())));
                }
            }
        }

        // queen side castle
        if !piece_to_move.has_moved() {
            let one_left = piece_to_move.get_occupied_field().get_offset_field(-1, 0).unwrap();
            let two_left = piece_to_move.get_occupied_field().get_offset_field(-2, 0).unwrap();
            let three_left = piece_to_move.get_occupied_field().get_offset_field(-3, 0).unwrap();
            let rook_field = piece_to_move.get_occupied_field().get_offset_field(-4, 0).unwrap();
            let possible_rook = state.get_piece_at(&rook_field);
            if possible_rook.is_some() {
                let rook = possible_rook.unwrap();
                if !rook.has_moved()
                    && !state.is_field_occupied(&one_left)
                    && !state.is_field_occupied(&two_left)
                    && !state.is_field_occupied(&three_left) {
                    moves.push_if_exists(Some(AllowedMove::new_composite_move(two_left, one_left, rook.clone())));
                }
            }
        }

        return moves;
    }
}

/**
other stuff
 */
pub fn create_move_component(piece_type: &PieceType) -> Box<dyn PieceMoveComponent> {
    match piece_type {
        PieceType::PAWN => Box::new(PawnMoveComponent {}),
        PieceType::ROOK => Box::new(RockMoveComponent {}),
        PieceType::BISHOP => Box::new(BishopMoveComponent {}),
        PieceType::KNIGHT => Box::new(KnightMoveComponent {}),
        PieceType::QUEEN => Box::new(QueenMoveComponent {}),
        PieceType::KING => Box::new(KingMoveComponent {}),
    }
}

// for testing
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



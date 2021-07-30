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
}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {
    fn get_all_allowed_moves(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut allowed_moves = vec!();
        allowed_moves.push_if_exists(self.get_move_ahead(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_first_move(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_left_capture(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_right_capture(chessboard, piece_to_move));

        return AllowedMoves { moves: allowed_moves };
    }
}

/**
pawn move
 */
// todo: enpassant
impl PawnMoveComponent {
    fn get_move_ahead(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedMove> {
        let allowed_move = AllowedMove::to_field(chessboard, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), 0);
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
        match AllowedMove::to_field(chessboard, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), col_offset) {
            None => { None }
            Some(allowed_field) => {
                if allowed_field.get_move_type() == MoveType::CAPTURE {
                    return Some(allowed_field)
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
        AllowedMoves {
            moves
        }
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
        AllowedMoves {
            moves
        }
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
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 2, 1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 2, -1));

        // move right
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -1, 2));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 1, 2));

        // move down
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -2, 1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -2, -1));

        // move left
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -1, -2));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 1, -2));

        return AllowedMoves { moves };
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
        AllowedMoves {
            moves
        }
    }
}

/**
King move
 */

pub struct KingMoveComponent {}

impl PieceMoveComponent for KingMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut moves = vec!();
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 1, 0));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 1, 1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 0, 1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -1, 1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -1, 0));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, -1, -1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 0, -1));
        moves.push_if_exists(AllowedMove::to_field(state, piece_to_move, 1, -1));

        return AllowedMoves { moves };
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


pub struct AllowedMoves {
    moves: Vec<AllowedMove>,
}

impl AllowedMoves {
    pub fn get_moves(&self) -> &Vec<AllowedMove> {
        &self.moves
    }

    fn is_move_allowed(&self, target: &FieldLogic) -> bool {
        self.moves.iter().map(|allowed_move| allowed_move.target.clone()).any(|allowed_target| &allowed_target == target)
    }

    fn get_allowed_move_to(&self, target: &FieldLogic) -> Option<AllowedMove> {
        for allowed_move in self.moves.iter() {
            if &allowed_move.target == target {
                return Some(allowed_move.clone());
            }
        }
        return None;
    }
}

pub struct AllowedMove {
    target: FieldLogic,
    capture: Option<PieceLogic>,
}

impl Clone for AllowedMove {
    fn clone(&self) -> Self {
        let new_target = self.target.clone();
        let new_capture = if self.capture.is_some() {
            Some(self.capture.as_ref().unwrap().make_duplicate())
        } else {
            None
        };

        AllowedMove {
            target: new_target,
            capture: new_capture,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}

#[derive(Eq, PartialEq)]
pub enum MoveType {
    MOVE,
    CAPTURE,
}

impl AllowedMove {
    fn new_capture(target: FieldLogic, captured_piece: PieceLogic) -> AllowedMove {
        AllowedMove { target, capture: Some(captured_piece) }
    }

    fn new_move(target: FieldLogic) -> AllowedMove {
        AllowedMove { target, capture: None }
    }

    fn to_field(chessboard: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Option<AllowedMove> {
        match piece_to_move.get_occupied_field().get_offset_field(col_offset, row_offset) {
            None => None,
            Some(target_field) => {
                let possible_other_piece = chessboard.get_piece_at(&target_field);
                match possible_other_piece {
                    None => {
                        Some(AllowedMove::new_move(target_field))
                    }
                    Some(other_piece) => {
                        if other_piece.get_side() != piece_to_move.get_side() {
                            Some(AllowedMove::new_capture(target_field, other_piece.make_duplicate()))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    fn get_moves_in_direction(state: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Vec<AllowedMove> {
        let mut blocked = false;
        let mut allowed_moves = vec!();
        let mut i = 0;

        while !blocked {
            i = i + 1;
            let possible_target = piece_to_move.get_occupied_field().get_offset_field(i * row_offset, i * col_offset);
            match possible_target {
                None => { blocked = true; }
                Some(target) => {
                    let possible_other_piece = state.get_piece_at(&target);
                    match possible_other_piece {
                        None => { allowed_moves.push(AllowedMove::new_move(target)) }
                        Some(other_piece) => {
                            if other_piece.get_side() != piece_to_move.get_side() {
                                allowed_moves.push(AllowedMove::new_capture(target, other_piece.make_duplicate()))
                            }
                            blocked = true;
                        }
                    }
                }
            }
        }
        return allowed_moves;
    }

    pub fn get_target(&self) -> &FieldLogic {
        &self.target
    }

    pub fn get_move_type(&self) -> MoveType {
        match self.capture {
            None => { MoveType::MOVE }
            Some(_) => { MoveType::CAPTURE }
        }
    }

    pub fn get_capture(&self) -> &Option<PieceLogic> {
        &self.capture
    }
}




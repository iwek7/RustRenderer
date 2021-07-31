use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::piece::PieceLogic;

pub struct AllowedMoves {
    moves: Vec<AllowedMove>,
}

impl AllowedMoves {
    pub fn new(moves: Vec<AllowedMove>) -> AllowedMoves {
        AllowedMoves {
            moves
        }
    }

    pub fn get_moves(&self) -> &Vec<AllowedMove> {
        &self.moves
    }

    pub fn is_move_allowed(&self, target: &FieldLogic) -> bool {
        self.moves.iter().map(|allowed_move| allowed_move.target.clone()).any(|allowed_target| &allowed_target == target)
    }

    pub fn get_allowed_move_to(&self, target: &FieldLogic) -> Option<AllowedMove> {
        for allowed_move in self.moves.iter() {
            if &allowed_move.target == target {
                return Some(allowed_move.clone());
            }
        }
        return None;
    }
}

#[derive(Eq, PartialEq)]
pub enum MoveType {
    MOVE,
    CAPTURE,
}


pub struct AllowedMove {
    target: FieldLogic,
    capture: Option<PieceLogic>,
    accompanying_move: Option<AccompanyingMove>,
}

impl AllowedMove {
    pub fn new_capture(target: FieldLogic, captured_piece: PieceLogic) -> AllowedMove {
        AllowedMove { target, capture: Some(captured_piece), accompanying_move: None }
    }

    pub fn new_move(target: FieldLogic) -> AllowedMove {
        AllowedMove { target, capture: None, accompanying_move: None }
    }

    pub fn new_composite_move(target: FieldLogic, accompanying_target: FieldLogic, accompanying_piece: PieceLogic) -> AllowedMove {
        AllowedMove { target, capture: None, accompanying_move: Some(AccompanyingMove::new(accompanying_target, accompanying_piece)) }
    }

    pub fn to_field(chessboard: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Option<AllowedMove> {
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

    pub fn get_moves_in_direction(state: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Vec<AllowedMove> {
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

    pub fn get_accompanying_move(&self) -> &Option<AccompanyingMove> {
        &self.accompanying_move
    }
}


impl Clone for AllowedMove {
    fn clone(&self) -> Self {
        let new_target = self.target.clone();
        let new_capture = if self.capture.is_some() {
            Some(self.capture.as_ref().unwrap().make_duplicate())
        } else {
            None
        };

        let new_accompanying_move = self.accompanying_move.clone();
        AllowedMove {
            target: new_target,
            capture: new_capture,
            accompanying_move: new_accompanying_move,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}


#[derive(Clone)]
pub struct AccompanyingMove {
    target: FieldLogic,
    piece: PieceLogic,
}

impl AccompanyingMove {
    fn new(target: FieldLogic, piece: PieceLogic) -> AccompanyingMove {
        AccompanyingMove {
            target,
            piece,
        }
    }

    pub fn get_target(&self) -> &FieldLogic {
        &self.target
    }

    pub fn get_piece(&self) -> &PieceLogic {
        &self.piece
    }
}



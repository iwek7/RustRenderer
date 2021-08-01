use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::piece::PieceLogic;

pub struct AllowedMoves {
    moves: Vec<AllowedAction>,
}

impl AllowedMoves {
    pub fn new(actions: Vec<AllowedAction>, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let filtered_moves: Vec<AllowedAction> = actions.clone().into_iter()
            .filter(|allowed_action| allowed_action.get_action_type() != ActionType::SUPPORT)
            .collect();
        AllowedMoves {
            moves: if state.is_in_check(piece_to_move.get_side()) {
                filtered_moves.clone().into_iter()
                    .filter(|allowed_move| {
                        let new_state = state.move_piece_to(piece_to_move.get_occupied_field(), allowed_move.get_target());
                        !new_state.is_in_check(piece_to_move.get_side())
                    })
                    .collect()
            } else {
                filtered_moves
            }
        }
    }

    pub fn get_moves(&self) -> &Vec<AllowedAction> {
        &self.moves
    }

    pub fn is_move_allowed(&self, target: &FieldLogic) -> bool {
        self.moves.iter().map(|allowed_move| allowed_move.target.clone()).any(|allowed_target| &allowed_target == target)
    }

    pub fn get_allowed_move_to(&self, target: &FieldLogic) -> Option<AllowedAction> {
        for allowed_move in self.moves.iter() {
            if &allowed_move.target == target {
                return Some(allowed_move.clone());
            }
        }
        return None;
    }
}

#[derive(Eq, PartialEq)]
pub enum ActionType {
    MOVE,
    CAPTURE,
    SUPPORT, // cant move but is supporting this field
}


pub struct AllowedAction {
    target: FieldLogic,
    capture: Option<PieceLogic>,
    accompanying_move: Option<AccompanyingMove>,
    is_support: bool,
}

impl AllowedAction {
    pub fn new_capture(target: FieldLogic, captured_piece: PieceLogic) -> AllowedAction {
        AllowedAction { target, capture: Some(captured_piece), accompanying_move: None, is_support: false }
    }

    pub fn new_move(target: FieldLogic) -> AllowedAction {
        AllowedAction { target, capture: None, accompanying_move: None, is_support: false }
    }

    pub fn new_composite_move(target: FieldLogic, accompanying_target: FieldLogic, accompanying_piece: PieceLogic) -> AllowedAction {
        AllowedAction { target, capture: None, accompanying_move: Some(AccompanyingMove::new(accompanying_target, accompanying_piece)), is_support: false }
    }

    pub fn new_support(target: FieldLogic) -> AllowedAction {
        AllowedAction { target, capture: None, accompanying_move: None, is_support: true }
    }

    pub fn movable_to_field(chessboard: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Option<AllowedAction> {
      match AllowedAction::action_to_field(chessboard, piece_to_move, row_offset, col_offset) {
          None => { None }
          Some(action) => {
              if action.get_action_type() == ActionType::SUPPORT {
                  return None;
              }
              Some(action)
          }
      }
    }

    pub fn action_to_field(chessboard: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Option<AllowedAction> {
        match piece_to_move.get_occupied_field().get_offset_field(col_offset, row_offset) {
            None => None,
            Some(target_field) => {
                let possible_other_piece = chessboard.get_piece_at(&target_field);
                match possible_other_piece {
                    None => {
                        Some(AllowedAction::new_move(target_field))
                    }
                    Some(other_piece) => {
                        if other_piece.get_side() != piece_to_move.get_side() {
                            Some(AllowedAction::new_capture(target_field, other_piece.make_duplicate()))
                        } else {
                            Some(AllowedAction::new_support(target_field))
                        }
                    }
                }
            }
        }
    }

    pub fn get_all_actions_in_direction(state: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Vec<AllowedAction> {
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
                        None => { allowed_moves.push(AllowedAction::new_move(target)) }
                        Some(other_piece) => {
                            match other_piece.get_side() != piece_to_move.get_side() {
                                true => allowed_moves.push(AllowedAction::new_capture(target, other_piece.make_duplicate())),
                                false => allowed_moves.push(AllowedAction::new_support(target))
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

    pub fn get_action_type(&self) -> ActionType {
        if self.capture.is_some() {
            ActionType::CAPTURE
        } else if self.is_support {
            ActionType::SUPPORT
        } else {
            ActionType::MOVE
        }
    }

    pub fn get_capture(&self) -> &Option<PieceLogic> {
        &self.capture
    }

    pub fn get_accompanying_move(&self) -> &Option<AccompanyingMove> {
        &self.accompanying_move
    }
}


impl Clone for AllowedAction {
    fn clone(&self) -> Self {
        let new_target = self.target.clone();
        let new_capture = if self.capture.is_some() {
            Some(self.capture.as_ref().unwrap().make_duplicate())
        } else {
            None
        };

        let new_accompanying_move = self.accompanying_move.clone();
        let new_is_support = self.is_support.clone();
        AllowedAction {
            target: new_target,
            capture: new_capture,
            accompanying_move: new_accompanying_move,
            is_support: new_is_support,
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



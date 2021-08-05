use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::piece::PieceLogic;

pub struct AllowedMoves {
    moves: Vec<AllowedAction>,
}

impl AllowedMoves {
    pub fn new(actions: Vec<AllowedAction>, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let filtered_moves: Vec<AllowedAction> = actions.clone().into_iter()
            .filter(|allowed_action| allowed_action.action_type.is_movable())
            .collect();
        AllowedMoves {
            moves: filtered_moves.clone().into_iter()
                .filter(|allowed_move| {
                    let new_state = state.move_piece_to(piece_to_move.get_occupied_field(), allowed_move.get_target());
                    !new_state.is_in_check(piece_to_move.get_side())
                })
                .collect()
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

#[derive(Clone)]
pub enum ActionType {
    MOVE,
    COMPOSITE_MOVE { accompanying_move: AccompanyingMove },
    CAPTURE { captured_piece: PieceLogic },
    SUPPORT,
    // cant move but is supporting this field
    PROMOTION,
    CAPTURE_PROMOTION { captured_piece: PieceLogic },
}

impl ActionType {
    fn is_movable(&self) -> bool {
        match self {
            ActionType::SUPPORT => { false }
            _ => true
        }
    }
}

// todo consider removing this object and just use enum with variable...
pub struct AllowedAction {
    target: FieldLogic,
    action_type: ActionType,
}

impl AllowedAction {
    pub fn new_capture(target: FieldLogic, captured_piece: PieceLogic) -> AllowedAction {
        AllowedAction { target, action_type: ActionType::CAPTURE { captured_piece } }
    }

    pub fn new_move(target: FieldLogic) -> AllowedAction {
        AllowedAction { target, action_type: ActionType::MOVE }
    }

    pub fn new_composite_move(target: FieldLogic, accompanying_target: FieldLogic, accompanying_piece: PieceLogic) -> AllowedAction {
        AllowedAction {
            target,
            action_type: ActionType::COMPOSITE_MOVE {
                accompanying_move: AccompanyingMove::new(accompanying_target, accompanying_piece)
            },
        }
    }

    pub fn new_support(target: FieldLogic) -> AllowedAction {
        AllowedAction { target, action_type: ActionType::SUPPORT }
    }

    pub fn new_promotion(target: FieldLogic) -> AllowedAction {
        AllowedAction { target, action_type: ActionType::PROMOTION }
    }

    pub fn new_capture_promotion(target: FieldLogic, captured_piece: PieceLogic) -> AllowedAction {
        AllowedAction { target, action_type: ActionType::CAPTURE_PROMOTION { captured_piece } }
    }

    pub fn movable_to_field(chessboard: &ChessboardState, piece_to_move: &PieceLogic, row_offset: i32, col_offset: i32) -> Option<AllowedAction> {
        match AllowedAction::action_to_field(chessboard, piece_to_move, row_offset, col_offset) {
            None => { None }
            Some(action) => {
                if action.get_action_type().is_movable() {
                    Some(action)
                } else {
                    None
                }
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

    pub fn get_action_type(&self) -> &ActionType {
        &self.action_type
    }
}


impl Clone for AllowedAction {
    fn clone(&self) -> Self {
        let new_target = self.target.clone();
        let new_action_type = self.action_type.clone();
        AllowedAction {
            target: new_target,
            action_type: new_action_type,
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



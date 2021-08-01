use crate::chess::allowed_move::{ActionType, AllowedAction, AllowedMoves};
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::infrastructure::{PieceType, VectorExtension};
use crate::chess::piece::PieceLogic;

/**
piece move trait
 */
pub trait PieceMoveComponent {
    fn is_move_allowed(&self, state: &ChessboardState, target_field: &FieldLogic, piece_to_move: &PieceLogic) -> Option<AllowedAction> {
        println!("Checking allowed move to {:?}", piece_to_move.get_occupied_field());
        // todo take into account supporting move
        return self.get_all_allowed_moves(state, piece_to_move).get_allowed_move_to(target_field);
    }

    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves;

    // todo this default is wrong, does not include pieces occupied by allies (which are technically `attacked`)
    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction>;
}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {
    fn get_all_allowed_moves(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut allowed_moves = vec!();
        allowed_moves.push_if_exists(self.get_move_ahead(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_first_move(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_left_capture(chessboard, piece_to_move));
        allowed_moves.push_if_exists(self.get_right_capture(chessboard, piece_to_move));
        return AllowedMoves::new(allowed_moves, chessboard, piece_to_move);
    }

    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        let mut attacks = vec!();
        attacks.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), -1));
        attacks.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), 1));
        return attacks;
    }
}

/**
pawn move
 */
// todo: enpassant
impl PawnMoveComponent {
    fn get_move_ahead(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedAction> {
        let allowed_move = AllowedAction::movable_to_field(chessboard, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), 0);
        return match allowed_move {
            None => { None }
            Some(real_move) => {
                if real_move.get_action_type() == ActionType::MOVE {
                    Some(real_move)
                } else {
                    None
                }
            }
        };
    }

    fn get_first_move(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedAction> {
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
                Some(AllowedAction::new_move(field_ahead))
            } else {
                None
            }
        }
    }

    fn get_left_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedAction> {
        self.get_capture(chessboard, piece_to_move, -1)
    }

    fn get_right_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic) -> Option<AllowedAction> {
        self.get_capture(chessboard, piece_to_move, 1)
    }

    fn get_capture(&self, chessboard: &ChessboardState, piece_to_move: &PieceLogic, col_offset: i32) -> Option<AllowedAction> {
        match AllowedAction::movable_to_field(chessboard, piece_to_move, piece_to_move.get_side().adjust_pawn_move_offset(&1), col_offset) {
            None => { None }
            Some(allowed_field) => {
                if allowed_field.get_action_type() == ActionType::CAPTURE {
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
pub struct RookMoveComponent {}

impl PieceMoveComponent for RookMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        AllowedMoves::new(self.get_all_actions(state, piece_to_move), state, piece_to_move)
    }

    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        self.get_all_actions(state, piece_to_move)
    }
}

impl RookMoveComponent {
    fn get_all_actions(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        let mut actions = vec!();
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 1, 0));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 0, 1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, -1, 0));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 0, -1));
        actions
    }
}

/**
bishop move
 */
pub struct BishopMoveComponent {}

impl PieceMoveComponent for BishopMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        AllowedMoves::new(self.get_all_actions(state, piece_to_move), state, piece_to_move)
    }

    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        self.get_all_actions(state, piece_to_move)
    }
}

impl BishopMoveComponent {
    fn get_all_actions(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        let mut actions = vec!();
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 1, 1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 1, -1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, -1, 1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, -1, -1));
        actions
    }
}

/**
Knight move
 */
pub struct KnightMoveComponent {}

impl PieceMoveComponent for KnightMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        return AllowedMoves::new(self.get_all_actions(state, piece_to_move), state, piece_to_move);
    }

    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        self.get_all_actions(state, piece_to_move)
    }
}

impl KnightMoveComponent {
    fn get_all_actions(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        let mut actions = vec!();
        // todo: write macro that takes all args and adds only not empty optionals

        // move up
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 2, 1));
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 2, -1));

        // move right
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -1, 2));
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 1, 2));

        // move down
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -2, 1));
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -2, -1));

        // move left
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -1, -2));
        actions.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 1, -2));
        actions
    }
}

/**
Queen move
 */

pub struct QueenMoveComponent {}

impl PieceMoveComponent for QueenMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        AllowedMoves::new(self.get_all_actions(state, piece_to_move), state, piece_to_move)
    }

    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        self.get_all_actions(state, piece_to_move)
    }
}

impl QueenMoveComponent {
    fn get_all_actions(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        let mut actions = vec!();
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 1, 1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 1, -1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, -1, 1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, -1, -1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 1, 0));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 0, 1));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, -1, 0));
        actions.append(&mut AllowedAction::get_all_actions_in_direction(state, piece_to_move, 0, -1));
        actions
    }
}

/**
King move
 */

pub struct KingMoveComponent {}

impl PieceMoveComponent for KingMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> AllowedMoves {
        let mut moves = vec!();
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 1, 0));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 1, 1));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 0, 1));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -1, 1));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -1, 0));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, -1, -1));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 0, -1));
        moves.push_if_exists(AllowedAction::action_to_field(state, piece_to_move, 1, -1));

        // todo: prevent casting when king is in check
        // castles
        if !piece_to_move.has_moved() {
            let attacked_fields = state.get_all_attacked_fields(piece_to_move.get_side());

            // king side castle
            let one_right = piece_to_move.get_occupied_field().get_offset_field(1, 0).unwrap();
            let two_right = piece_to_move.get_occupied_field().get_offset_field(2, 0).unwrap();
            let rook_field = piece_to_move.get_occupied_field().get_offset_field(3, 0).unwrap();
            let possible_right_rook = state.get_piece_at(&rook_field);
            if possible_right_rook.is_some() {
                let rook = possible_right_rook.unwrap();
                if !rook.has_moved()
                    && !state.is_field_occupied(&one_right)
                    && !state.is_field_occupied(&two_right)
                    && !attacked_fields.contains(&one_right)
                    && !attacked_fields.contains(&two_right)
                {
                    moves.push_if_exists(Some(AllowedAction::new_composite_move(two_right, one_right, rook.clone())));
                }
            }

            // queen side castle
            let one_left = piece_to_move.get_occupied_field().get_offset_field(-1, 0).unwrap();
            let two_left = piece_to_move.get_occupied_field().get_offset_field(-2, 0).unwrap();
            let three_left = piece_to_move.get_occupied_field().get_offset_field(-3, 0).unwrap();
            let rook_field = piece_to_move.get_occupied_field().get_offset_field(-4, 0).unwrap();
            let possible_left_rook = state.get_piece_at(&rook_field);
            if possible_left_rook.is_some() {
                let rook = possible_left_rook.unwrap();
                if !rook.has_moved()
                    && !state.is_field_occupied(&one_left)
                    && !state.is_field_occupied(&two_left)
                    && !state.is_field_occupied(&three_left)
                    && !attacked_fields.contains(&one_left)
                    && !attacked_fields.contains(&two_left) {
                    moves.push_if_exists(Some(AllowedAction::new_composite_move(two_left, one_left, rook.clone())));
                }
            }
        }

        let attacked_fields = state.get_all_attacked_fields(piece_to_move.get_side());
        return AllowedMoves::new(moves.iter()
                                     .filter(|allowed_move| !attacked_fields.contains(allowed_move.get_target()))
                                     .cloned()
                                     .collect(),
                                 state, piece_to_move,
        );
    }

    fn get_all_attacks(&self, state: &ChessboardState, piece_to_move: &PieceLogic) -> Vec<AllowedAction> {
        let mut attacked_fields = vec!();

        AllowedAction::action_to_field(state, piece_to_move,1, 0);
        AllowedAction::action_to_field(state, piece_to_move,1, 1);
        AllowedAction::action_to_field(state, piece_to_move,0, 1);
        AllowedAction::action_to_field(state, piece_to_move,-1, 1);
        AllowedAction::action_to_field(state, piece_to_move,-1, 0);
        AllowedAction::action_to_field(state, piece_to_move,-1, -1);
        AllowedAction::action_to_field(state, piece_to_move,0, -1);
        AllowedAction::action_to_field(state, piece_to_move,1, -1);

        return attacked_fields;
    }
}


/**
other stuff
 */
pub fn create_move_component(piece_type: &PieceType) -> Box<dyn PieceMoveComponent> {
    match piece_type {
        PieceType::PAWN => Box::new(PawnMoveComponent {}),
        PieceType::ROOK => Box::new(RookMoveComponent {}),
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



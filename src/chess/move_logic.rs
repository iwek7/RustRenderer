use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;
use crate::chess::infrastructure::PieceType;

pub trait PieceMoveComponent {
    fn is_move_allowed(&self, state: &ChessboardState, target_field: &FieldLogic, occupied_field: &FieldLogic) -> bool {
        println!("Target field is {:?}", target_field);
        self.get_all_allowed_moves(state, occupied_field).contains(target_field)
    }
    fn get_all_allowed_moves(&self, state: &ChessboardState, occupied_field: &FieldLogic) -> Vec<FieldLogic>;
}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, occupied_field: &FieldLogic) -> Vec<FieldLogic> {
        vec!(
            FieldLogic::from_string("B1"),
            FieldLogic::from_string("B2"),
            FieldLogic::from_string("B3"),
        )
    }
}

pub struct RockMoveComponent {}

impl PieceMoveComponent for RockMoveComponent {
    fn get_all_allowed_moves(&self, state: &ChessboardState, occupied_field: &FieldLogic) -> Vec<FieldLogic> {
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
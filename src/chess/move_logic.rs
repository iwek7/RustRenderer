use crate::chess::infrastructure::PieceType;
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::FieldLogic;

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
        todo!()
    }
}


pub fn create_move_component(piece_type: &PieceType) -> Box<dyn PieceMoveComponent> {
    Box::new(PawnMoveComponent {})
}

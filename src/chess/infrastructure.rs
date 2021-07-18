pub trait Draggable {
    // todo: this really should not accept opengl coords, all should happen in world coord space
    fn is_mouse_over(&self, mouse_pos_opengl: &(f32, f32)) -> bool;
    fn handle_start_drag(&mut self);
    fn handle_drop(&mut self, final_pos: Option<(f32, f32)>);

    fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32));
}

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
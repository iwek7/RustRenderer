use crate::{render_gl, create_rect_coords_in_opengl_space};
use crate::opengl_context::OpenglContext;
use crate::maths::quadrangle::Quadrangle;
use crate::texture::Texture;
use std::borrow::Borrow;
use crate::maths::vertex::VertexTextured;
use crate::maths::triangle::Drawable;
use crate::chess::infrastructure::{Draggable, PieceType, Side};
use crate::maths::shapes_common::Area;

pub struct Piece<'a> {
    piece_type: PieceType,
    quad: Quadrangle<'a, VertexTextured>,
    move_component: Box<dyn PieceMoveComponent>,
    is_dragged: bool,
    initial_drag_pos_opengl: (f32, f32, f32),
}

impl<'a> Drawable for Piece<'a> {
    fn render(&self) {
        self.quad.render()
    }
}

impl<'a> Draggable for Piece<'a> {
    fn is_mouse_over(&self, mouse_coords_opengl: &(f32, f32)) -> bool {
        self.quad.contains_point(mouse_coords_opengl)
    }

    fn handle_start_drag(&mut self) {
        self.is_dragged = true;
        let pos = self.quad.get_pos();
        self.initial_drag_pos_opengl = self.quad.get_pos();
    }

    fn handle_drop(&mut self, final_pos: Option<(f32, f32)>) {
        if self.is_dragged {
            match final_pos {
                None => {
                    self.quad.move_to(&self.initial_drag_pos_opengl);
                    self.is_dragged = false
                } //comeback
                Some(_) => {
                    let unwr = final_pos.unwrap();
                    self.quad.move_to(&(unwr.0, unwr.1, 0.0));
                    self.is_dragged = false
                }
            }
        }
    }

    fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32)) {
        if self.is_dragged {
            self.quad.move_by(drag_offset.0, drag_offset.1, 0.0)
        }
    }
}

static BLACK_ROW: u32 = 0;
static WHITE_ROW: u32 = 1;
static PAWN_COL: u32 = 5;
static ROOK_COL: u32 = 4;
static KNIGHT_COL: u32 = 3;
static BISHOP_COL: u32 = 2;
static QUEEN_COL: u32 = 1;
static KING_COL: u32 = 0;

pub struct PieceFactory<'a> {
    shader: &'a render_gl::Program,
    opengl_context: &'a OpenglContext,
}

impl<'a> PieceFactory<'a> {
    pub fn new(opengl_context: &'a OpenglContext, shader: &'a render_gl::Program) -> PieceFactory<'a> {
        return PieceFactory {
            shader,
            opengl_context,
        };
    }

    pub fn init_piece(&self, piece_type: PieceType, side: Side, pieces_sheet: &'a Texture, pos: (i32, i32, i32), size: (i32, i32)) -> Piece<'a> {
        let sheet_coords = PieceFactory::get_sprite_sheet_coords(&piece_type, &side);
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(
                &self.opengl_context,
                pos,
                size,
                pieces_sheet.topology.get_sprite_coords(sheet_coords.0, sheet_coords.1).unwrap().clone().borrow(),
            ),
            [0, 1, 3, 1, 2, 3],
            self.shader,
            Some(pieces_sheet),
        );

        let move_component = PawnMoveComponent {};
        return Piece {
            piece_type,
            quad,
            move_component: Box::new(move_component),
            is_dragged: false,
            initial_drag_pos_opengl: (0.0, 0.0, 0.0),
        };
    }


    fn get_sprite_sheet_coords(piece_type: &PieceType, side: &Side) -> (u32, u32) {
        (
            match side {
                Side::BLACK => BLACK_ROW,
                Side::WHITE => WHITE_ROW
            },
            match piece_type {
                PieceType::PAWN => PAWN_COL,
                PieceType::KNIGHT => KNIGHT_COL,
                PieceType::BISHOP => BISHOP_COL,
                PieceType::ROOK => ROOK_COL,
                PieceType::QUEEN => QUEEN_COL,
                PieceType::KING => KING_COL
            }
        )
    }
}

pub trait PieceMoveComponent {}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {}


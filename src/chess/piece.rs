use std::borrow::Borrow;

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::{Field, FieldData};
use crate::chess::infrastructure::{PieceType, Side};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;

pub struct Piece<'a> {
    piece_type: PieceType,
    quad: Quadrangle<'a, VertexTextured>,
    pub move_component: Box<dyn PieceMoveComponent>,
    initial_drag_pos_opengl: (f32, f32, f32),
}

impl<'a> Drawable for Piece<'a> {
    fn render(&self) {
        self.quad.render()
    }
}

impl<'a> Piece<'a> {
    pub fn is_mouse_over(&self, mouse_coords_opengl: &(f32, f32)) -> bool {
        self.quad.contains_point(mouse_coords_opengl)
    }

    pub fn handle_start_drag(&mut self) {
        self.initial_drag_pos_opengl = self.quad.get_pos();
    }

    pub fn return_to_initial_pos(&mut self) {
        self.quad.move_to(&self.initial_drag_pos_opengl);
    }

    pub fn handle_drop(&mut self, context: &OpenglContext, target_field: FieldData, pos: (i32, i32, i32), chessboard_state: &ChessboardState) {
        if self.move_component.is_move_allowed(chessboard_state, &target_field) {
            let opengl_pos = context.sdl_window_to_opengl_space3(&pos);
            self.quad.move_to(&(opengl_pos.0, opengl_pos.1, 0.0));
        } else {
            self.quad.move_to(&self.initial_drag_pos_opengl);
        }
    }

    pub fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32)) {
        self.quad.move_by(drag_offset.0, drag_offset.1, 0.0)
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

pub trait PieceMoveComponent {
    fn is_move_allowed(&self, state: &ChessboardState, target_field: &FieldData) -> bool;
    fn get_all_allowed_moves(&self, state: ChessboardState) -> Vec<FieldData>;
}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {
    fn is_move_allowed(&self, state: &ChessboardState, target_field: &FieldData) -> bool {
        !target_field.col != 0
    }

    fn get_all_allowed_moves(&self, state: ChessboardState) -> Vec<FieldData> {
        vec!(
            FieldData::from_string("B2"),
            FieldData::from_string("C4")


        )
    }
}


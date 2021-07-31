use core::fmt;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::{Field, FieldLogic};
use crate::chess::infrastructure::{PieceType, Side};
use crate::chess::move_logic::{PieceMoveComponent};
use crate::chess::move_logic::create_move_component;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;
use crate::chess::allowed_move::{AllowedMove, AllowedMoves};

pub struct Piece<'a> {
    pub logic: PieceLogic,
    quad: Quadrangle<'a, VertexTextured>,
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

    pub fn handle_drop(&mut self, context: &OpenglContext, target_field: FieldLogic, pos: (i32, i32, i32), chessboard_state: &ChessboardState) -> Option<AllowedMove> {
        println!("Dropping piece at field {:?} position {:?}", target_field, pos);
        return match self.logic.move_component.is_move_allowed(chessboard_state, &target_field, &self.logic) {
            None => {
                self.quad.move_to(&self.initial_drag_pos_opengl);
                None
            }
            Some(allowed_move) => {
                self.force_move(context, target_field.clone(), pos);
                Some(allowed_move)
            }
        }
    }

    pub fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32)) {
        self.quad.move_by(drag_offset.0, drag_offset.1, 0.0)
    }

    pub fn force_move(&mut self, context: &OpenglContext, target_field: FieldLogic, pos: (i32, i32, i32)) {
        let opengl_pos = context.engine_to_opengl_space(&pos);
        self.quad.move_to(&opengl_pos);
        self.logic = self.logic.move_to(&target_field);
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

    pub fn init_piece(&self, piece_type: PieceType, side: Side, pieces_sheet: &'a Texture, field: &Field, size: (i32, i32)) -> Piece<'a> {
        let sheet_coords = PieceFactory::get_sprite_sheet_coords(&piece_type, &side);
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(
                &self.opengl_context,
                field.get_position_3d(),
                size,
                pieces_sheet.topology.get_sprite_coords(sheet_coords.0, sheet_coords.1).unwrap().clone().borrow(),
            ),
            [0, 1, 3, 1, 2, 3],
            self.shader,
            Some(pieces_sheet),
        );
        let move_component = create_move_component(&piece_type);
        return Piece {
            logic: PieceLogic {
                piece_type,
                move_component,
                side,
                occupied_field: field.logic.clone(),
                moved: false,
            },
            quad,
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

pub struct PieceLogic {
    move_component: Box<dyn PieceMoveComponent>, // todo this feels so wrong here and causes so much issues..., maybe it should somehow part of PieceType? or monad?
    piece_type: PieceType,
    side: Side,
    occupied_field: FieldLogic,
    moved: bool,
}

impl PieceLogic {
    pub fn get_all_allowed_moves(&self, state: &ChessboardState) -> AllowedMoves {
        self.move_component.get_all_allowed_moves(state, &self)
    }

    pub fn get_all_attacked_fields(&self, state: &ChessboardState) -> Vec<FieldLogic> {
        self.move_component.get_all_attacked_fields(state, &self)
    }

    pub fn move_to(&self, target_field: &FieldLogic) -> PieceLogic {
        // todo: validate if move is legal
        PieceLogic {
            move_component: create_move_component(&self.piece_type),
            piece_type: self.piece_type.clone(),
            occupied_field: target_field.clone(),
            moved: true,
            side: self.side.clone(),
        }
    }

    // todo: remove this in favour of clone
    pub fn make_duplicate(&self) -> PieceLogic {
        PieceLogic {
            move_component: create_move_component(&self.piece_type),
            piece_type: self.piece_type.clone(),
            occupied_field: self.occupied_field.clone(),
            moved: self.moved,
            side: self.side.clone(),
        }
    }

    pub fn get_occupied_field(&self) -> &FieldLogic {
        &self.occupied_field
    }

    pub fn has_moved(&self) -> bool {
        self.moved.clone()
    }

    pub fn get_side(&self) -> &Side {
        &self.side
    }
}

impl Clone for PieceLogic {
    fn clone(&self) -> Self {
        self.make_duplicate()
    }

    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}

impl Display for PieceLogic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Piece of type {:?}, side: {:?}, occupiedField {:?}", self.piece_type, self.side, self.occupied_field)
    }
}

impl PartialEq for PieceLogic {
    fn eq(&self, other: &Self) -> bool {
        self.moved == other.moved && self.occupied_field == other.occupied_field && self.side == other.side && self.piece_type == other.piece_type
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}



use core::fmt;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::chess::allowed_move::{AllowedAction, AllowedMoves};
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::{Field, FieldLogic};
use crate::chess::infrastructure::{PieceType, Side};
use crate::chess::move_logic::create_move_component;
use crate::chess::move_logic::PieceMoveComponent;
use crate::create_rect_coords_deprecated;
use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::TexturedVertexData;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Texture;
use crate::engine::rendering::material::Material;

pub struct Piece {
    pub logic: PieceLogic,
    quad: Quadrangle<TexturedVertexData>,
    initial_drag_pos_opengl: (f32, f32, f32),
}

impl Drawable for Piece {
    fn render(&mut self, render_util: &RenderUtil) {
        self.quad.render(render_util)
    }
}

impl Piece {
    pub fn is_mouse_over(&self, world_mouse_position: &glam::Vec3) -> bool {
        self.quad.contains_point(&(world_mouse_position.x, world_mouse_position.y))
    }

    pub fn handle_start_drag(&mut self) {
        println!("Started dragging piece {}", self.logic);
        self.initial_drag_pos_opengl = self.quad.get_pos();
    }

    pub fn return_to_initial_pos(&mut self) {
        println!("Resetting position of piece {}", self.logic);
        self.quad.move_to(&self.initial_drag_pos_opengl);
    }

    pub fn handle_drop(&mut self, target_field: FieldLogic, pos: (f32, f32, f32), chessboard_state: &ChessboardState) -> Option<AllowedAction> {
        println!("Dropping piece at field {:?} position {:?}", target_field, pos);
        return match self.logic.move_component.is_move_allowed(chessboard_state, &target_field, &self.logic) {
            None => {
                self.quad.move_to(&self.initial_drag_pos_opengl);
                None
            }
            Some(allowed_move) => {
                self.force_move(target_field.clone(), pos);
                Some(allowed_move)
            }
        };
    }

    pub fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32)) {
        self.quad.move_by(drag_offset.0, drag_offset.1, 0.0)
    }

    pub fn force_move(&mut self, target_field: FieldLogic, pos: (f32, f32, f32)) {
        self.quad.move_to(&pos);
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

pub struct PieceFactory {
    piece_material: Material,
}

impl PieceFactory {
    pub fn new(material: Material) -> PieceFactory {
        return PieceFactory {
            piece_material: material
        };
    }

    pub fn init_piece(&self, piece_type: PieceType, side: Side, pieces_sheet: Rc<Texture>, field: &Field, size: (f32, f32)) -> Piece {
        let sheet_coords = PieceFactory::get_sprite_sheet_coords(&piece_type, &side);
        let f_pos = field.get_position_3d();
        // todo: all types here should be either i32 or f32
        let q_pos = (f_pos.0 as f32, f_pos.1 as f32, 0 as f32);
        let quad = Quadrangle::new(
            create_rect_coords_deprecated(
                q_pos,
                size,
                pieces_sheet.topology.get_sprite_coords(sheet_coords.0, sheet_coords.1).unwrap().clone().borrow(),
            ),
            [0, 1, 3, 1, 2, 3],
            self.piece_material.clone(),
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
    // todo this feels so wrong here and causes so much issues..., maybe it should somehow part of PieceType? or monad?
    move_component: Box<dyn PieceMoveComponent>,
    piece_type: PieceType,
    side: Side,
    occupied_field: FieldLogic,
    moved: bool,
}

impl PieceLogic {
    pub fn get_all_allowed_moves(&self, state: &ChessboardState) -> AllowedMoves {
        self.move_component.get_all_allowed_moves(state, &self)
    }

    pub fn get_all_attacked_fields(&self, state: &ChessboardState) -> Vec<AllowedAction> {
        self.move_component.get_all_attacks(state, &self)
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

    pub fn get_type(&self) -> &PieceType { &self.piece_type }
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



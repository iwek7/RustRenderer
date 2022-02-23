use core::fmt;
use std::fmt::{Display, Formatter};

use crate::chess::allowed_move::{AllowedAction, AllowedMoves};
use crate::chess::chessboard::ChessboardState;
use crate::chess::field::{Field, FieldLogic};
use crate::chess::infrastructure::{PieceType, Side};
use crate::chess::move_logic::create_move_component;
use crate::chess::move_logic::PieceMoveComponent;
use crate::engine::api::game_object::{BaseGameObject, GameObject};
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::TexturedVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::{Sprite};
use crate::engine::rendering::material::Material;

pub struct Piece {
    base_game_object: BaseGameObject,
    pub logic: PieceLogic,
    rect: Rectangle<TexturedVertexDataLayout>,
    initial_drag_pos_opengl: glam::Vec3,
}

impl GameObject for Piece {
    fn render(&mut self, render_util: &RenderUtil) {
        self.rect.render(render_util)
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}

impl Piece {
    pub fn is_mouse_over(&self, world_mouse_position: &glam::Vec3) -> bool {
        self.rect.contains_point(&(world_mouse_position.x, world_mouse_position.y))
    }

    pub fn handle_start_drag(&mut self) {
        println!("Started dragging piece {}", self.logic);
        self.initial_drag_pos_opengl = *self.rect.get_pos();
    }

    pub fn return_to_initial_pos(&mut self) {
        println!("Resetting position of piece {}", self.logic);
        self.rect.move_to(self.initial_drag_pos_opengl.clone());
    }

    pub fn handle_drop(&mut self, target_field: FieldLogic, pos: glam::Vec3, chessboard_state: &ChessboardState) -> Option<AllowedAction> {
        println!("Dropping piece at field {:?} position {:?}", target_field, pos);
        return match self.logic.move_component.is_move_allowed(chessboard_state, &target_field, &self.logic) {
            None => {
                self.rect.move_to(self.initial_drag_pos_opengl.clone());
                None
            }
            Some(allowed_move) => {
                self.force_move(target_field.clone(), pos);
                Some(allowed_move)
            }
        };
    }

    pub fn handle_drag_pointer_move(&mut self, drag_offset: &glam::Vec3) {
        self.rect.move_by(*drag_offset)
    }

    pub fn force_move(&mut self, target_field: FieldLogic, pos: glam::Vec3) {
        self.rect.move_to(pos);
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

    pub fn init_piece(&self, piece_type: PieceType, side: Side, pieces_sheet: Sprite, field: &Field, size:glam::Vec2) -> Piece {
        let sheet_coords = PieceFactory::get_sprite_sheet_coords(&piece_type, &side);
        let pos = field.get_position_3d();

        let rect = Rectangle::new_from_spritesheet(
            &pos,
            &size,
            self.piece_material.clone(),
            pieces_sheet.clone(),
            sheet_coords.0,
            sheet_coords.1);

        let move_component = create_move_component(&piece_type);
        return Piece {
            base_game_object: BaseGameObject::new(),
            logic: PieceLogic {
                piece_type,
                move_component,
                side,
                occupied_field: field.logic.clone(),
                moved: false,
            },
            rect,
            initial_drag_pos_opengl: glam::vec3(0.0, 0.0, 0.0),
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



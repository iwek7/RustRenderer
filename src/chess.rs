use std::borrow::Borrow;

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::mouse_drag_controller::{Draggable, MouseDragController};
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;

pub struct SpriteSheet {
    sprite_sheet: Texture,
}

pub struct Piece<'a> {
    piece_type: PieceType,
    quad: Quadrangle<'a, VertexTextured>,
    move_component: Box<dyn PieceMoveComponent>,
}

impl<'a> Draggable for Piece<'a> {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool {
        self.quad.is_mouse_over(mouse_pos)
    }

    fn handle_start_drag(&mut self) {
        self.quad.handle_start_drag()
    }

    fn handle_drop(&mut self) {
        self.quad.handle_drop()
    }

    fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32)) {
        self.quad.handle_drag_pointer_move(drag_offset)
    }
}


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


static BLACK_ROW: u32 = 0;
static WHITE_ROW: u32 = 1;
static PAWN_COL: u32 = 5;
static ROOK_COL: u32 = 4;
static KNIGHT_COL: u32 = 3;
static BISHOP_COL: u32 = 2;
static QUEEN_COL: u32 = 1;
static KING_COL: u32 = 0;

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


impl<'a> Drawable for Piece<'a> {
    fn render(&self) {
        self.quad.render()
    }
}


pub trait PieceMoveComponent {}

pub struct PawnMoveComponent {}

impl PieceMoveComponent for PawnMoveComponent {}


pub struct Chessboard<'a> {
    board: Quadrangle<'a, VertexTextured>,
    pieces: Vec<Piece<'a>>,
    piece_factory: PieceFactory<'a>,
    tx: &'a Texture,
    mouse_drag_controller: MouseDragController,
    field_size: u32,
    position: (i32, i32, i32),
}

impl<'a> Drawable for Chessboard<'a> {
    fn render(&self) {
        self.board.render();
        self.pieces.iter().for_each(|piece| { piece.render() })
    }
}

impl<'a> Chessboard<'a> {
    pub fn new(chessboard_texture: &'a Texture, opengl_context: &'a OpenglContext, shader: &'a render_gl::Program) -> Chessboard<'a> {
        let mut mouse_drag_controller = MouseDragController::new();
        let field_size = 87;
        let position = (100, 0, 0);
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(&opengl_context, position.clone(), (87 * 8, 87 * 8), &chessboard_texture.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            &shader,
            Some(&chessboard_texture),
        );

        let piece_factory = PieceFactory::new(opengl_context, shader);

        return Chessboard {
            board: quad,
            pieces: vec!(),
            piece_factory,
            tx: chessboard_texture,
            mouse_drag_controller,
            field_size,
            position,
        };
    }

    pub fn init_pieces(&mut self, pieces_sheet: &'a Texture) {
        let piece_size = (self.field_size as i32, self.field_size as i32);
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::WHITE, pieces_sheet, self.get_field_position("A1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::WHITE, pieces_sheet, self.get_field_position("B1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::WHITE, pieces_sheet, self.get_field_position("C1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::QUEEN, Side::WHITE, pieces_sheet, self.get_field_position("D1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KING, Side::WHITE, pieces_sheet, self.get_field_position("E1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::WHITE, pieces_sheet, self.get_field_position("F1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::WHITE, pieces_sheet, self.get_field_position("G1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::WHITE, pieces_sheet, self.get_field_position("H1"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("A2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("B2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("C2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("D2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("E2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("F2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("G2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_position("H2"), piece_size));


        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::BLACK, pieces_sheet, self.get_field_position("A8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::BLACK, pieces_sheet, self.get_field_position("B8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::BLACK, pieces_sheet, self.get_field_position("C8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::QUEEN, Side::BLACK, pieces_sheet, self.get_field_position("D8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KING, Side::BLACK, pieces_sheet, self.get_field_position("E8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::BLACK, pieces_sheet, self.get_field_position("F8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::BLACK, pieces_sheet, self.get_field_position("G8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::BLACK, pieces_sheet, self.get_field_position("H8"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("A7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("B7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("C7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("D7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("E7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("F7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("G7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_position("H7"), piece_size));

    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event, mouse_pos: &(f32, f32)) {
        self.mouse_drag_controller.handle_event(event, mouse_pos, &mut self.pieces)
    }

    fn get_field_position(&self, field_str: &str) -> (i32, i32, i32) {
        let pos = Chessboard::parse_field_name(field_str);
        (
            pos.0 as i32 * self.field_size as i32 + self.position.0,
            pos.1 as i32 * self.field_size as i32 + self.position.1,
            0
        )
    }

    fn parse_field_name(field_str: &str) -> (u32, u32) {
        let col = match field_str.chars().nth(0).unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => panic!(format!("Unparsable chess field number {}", field_str))
        } as u32;
        let row = field_str.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;
        (col, row)
    }
}

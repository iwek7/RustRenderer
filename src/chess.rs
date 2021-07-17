use std::borrow::Borrow;

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::mouse_drag_controller::{Draggable, MouseDragController};
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;

static BLACK_ROW: u32 = 0;
static WHITE_ROW: u32 = 1;
static PAWN_COL: u32 = 0;
static ROOK_COL: u32 = 1;
static KNIGHT_COL: u32 = 2;
static BISHOP_COL: u32 = 3;
static QUEEN_COL: u32 = 4;
static KING_COL: u32 = 5;

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

    pub fn init_piece(&self, piece_type: PieceType, pieces_sheet: &'a Texture, pos: (i32, i32, i32), size: (i32, i32) ) -> Piece<'a> {
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(
                &self.opengl_context,
                pos,
                size,
                pieces_sheet.topology.get_sprite_coords(1, 1).unwrap().clone().borrow(),
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
}

pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KIND,
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
    field_size: f32
}

impl<'a> Drawable for Chessboard<'a> {
    fn render(&self) {
        self.board.render();
        self.pieces.iter().for_each(|piece| { piece.render() })
    }
}

impl<'a> Chessboard<'a> {
    pub fn new(chessboard_texture: &'a Texture, opengl_context: &'a OpenglContext, shader: &'a render_gl::Program, field_size: f32) -> Chessboard<'a> {
        let mut mouse_drag_controller = MouseDragController::new();
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(&opengl_context, (100, 0, 0), (700, 700), &chessboard_texture.topology.get_sprite_coords(0, 0).unwrap()),
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
            field_size
        };
    }

    pub fn init_pieces(&mut self, pieces_sheet: &'a Texture) {
        let piece = self.piece_factory.init_piece(PieceType::PAWN, pieces_sheet, (50, 100, 0), (self.field_size as i32, self.field_size as i32));
        self.pieces.push(piece);
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event, mouse_pos: &(f32, f32)) {
        self.mouse_drag_controller.handle_event(event, mouse_pos, &mut self.pieces)
    }
}

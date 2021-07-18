use std::borrow::Borrow;

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::shapes_common::Area;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;

pub struct SpriteSheet {
    sprite_sheet: Texture,
}

pub struct Piece<'a> {
    piece_type: PieceType,
    quad: Quadrangle<'a, VertexTextured>,
    move_component: Box<dyn PieceMoveComponent>,
    is_dragged: bool,
    initial_drag_pos_opengl: (f32, f32, f32),
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
    field_size: u32,
    board_size: u32,
    position: (i32, i32, i32),
    prev_mouse_pos: (f32, f32),
}

impl<'a> Drawable for Chessboard<'a> {
    fn render(&self) {
        self.board.render();
        self.pieces.iter().for_each(|piece| { piece.render() })
    }
}

impl<'a> Chessboard<'a> {
    pub fn new(chessboard_texture: &'a Texture, opengl_context: &'a OpenglContext, shader: &'a render_gl::Program) -> Chessboard<'a> {
        let field_size = 87;
        let board_size = field_size * 8;
        let position = (100, 0, 0);
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(&opengl_context, position.clone(), (board_size, board_size), &chessboard_texture.topology.get_sprite_coords(0, 0).unwrap()),
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
            field_size: field_size as u32,
            board_size: board_size as u32,
            position,
            prev_mouse_pos: (0.0, 0.0),
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

    /**
    iterating over all those draggables is veeery inefficient
    but I can't hold reference to currently dragged object here
    as it violates only one mutable ref rule
     **/
    // todo holy shit we operate in both coordinate systems at the same time...
    // todo even context is needed here to translate them...
    // todo horror
    pub fn handle_event(&mut self, event: &sdl2::event::Event, mouse_coords_px: &(i32, i32), mouse_coords_opengl: &(f32, f32), context: &OpenglContext) {
        let prev_mouse_pos = self.prev_mouse_pos.clone();
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                for obj in self.pieces.iter_mut() {
                    if obj.is_mouse_over(mouse_coords_opengl) {
                        obj.handle_start_drag()
                    }
                }
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                let final_pos = match self.get_field_coords_by_point(mouse_coords_px) {
                    // out of bounds of chessboard, set to initial file
                    None => { None }
                    Some(coords) => {
                        let final_pos_opengl = context.sdl_window_to_opengl_space(
                            &(
                                coords.0 as i32 * self.field_size as i32 + self.position.0,
                                coords.1 as i32 * self.field_size as i32 + self.position.1,
                            )
                        );
                        Some((final_pos_opengl.0, final_pos_opengl.1))
                    }
                };
                self.pieces.iter_mut().for_each(|piece| {
                    piece.handle_drop(final_pos);
                })
            }
            sdl2::event::Event::MouseMotion { .. } => {
                let drag_offset = &(
                    (mouse_coords_opengl.0 - self.prev_mouse_pos.0) as f32,
                    (mouse_coords_opengl.1 - self.prev_mouse_pos.1) as f32
                );
                self.pieces.iter_mut()
                    .for_each(|it| { it.handle_drag_pointer_move(drag_offset) });
            }

            _ => {}
        }
        self.prev_mouse_pos = mouse_coords_opengl.clone()
    }


    fn get_field_position(&self, field_str: &str) -> (i32, i32, i32) {
        let pos = Chessboard::parse_field_name(field_str);
        (
            pos.0 as i32 * self.field_size as i32 + self.position.0,
            pos.1 as i32 * self.field_size as i32 + self.position.1,
            0
        )
    }

    fn get_field_coords_by_point(&self, point: &(i32, i32)) -> Option<(u32, u32)> {
        if point.0 < self.position.0 ||
            point.0 as i32 > self.position.0 + self.board_size as i32 ||
            point.1 < self.position.1 ||
            point.1 as i32 > self.position.1 + self.board_size as i32 {
            return None;
        }
        return Some(
            (
                ((point.0 as i32 - self.position.0) / self.field_size as i32) as u32,
                ((point.1 as i32 - self.position.1) / self.field_size as i32) as u32
            )
        );
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

pub trait Draggable {
    // todo: this really should not accept opengl coords, all should happen in world coord space
    fn is_mouse_over(&self, mouse_pos_opengl: &(f32, f32)) -> bool;
    fn handle_start_drag(&mut self);
    fn handle_drop(&mut self, final_pos: Option<(f32, f32)>);

    fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32));
}
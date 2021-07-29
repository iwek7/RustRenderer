use std::collections::HashMap;

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::chess::field::{Field, FieldLogic};
use crate::chess::infrastructure::{PieceType, Side};
use crate::chess::piece::{Piece, PieceFactory, PieceLogic};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;

pub struct Chessboard<'a> {
    board: Quadrangle<'a, VertexTextured>,
    pieces: Vec<Piece<'a>>,
    piece_factory: PieceFactory<'a>,
    tx: &'a Texture,
    field_size: u32,
    board_size: u32,
    position: (i32, i32, i32),
    prev_mouse_pos: (f32, f32),
    fields: Vec<Vec<Field<'a>>>,
    dragger_piece: Option<usize>,
}

impl<'a> Chessboard<'a> {
    pub fn new(chessboard_texture: &'a Texture, opengl_context: &'a OpenglContext,
               shader: &'a render_gl::Program, possible_move_shader: &'a render_gl::Program) -> Chessboard<'a> {
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

        let mut fields = Vec::new();
        for row_idx in 0..8 as u32 {
            let mut row = Vec::new();
            for col_idx in 0..8 as u32 {
                row.push(Field::new(
                    col_idx,
                    row_idx,
                    col_idx as i32 * field_size + position.0,
                    row_idx as i32 * field_size + position.1,
                    field_size,
                    possible_move_shader,
                    opengl_context,
                ));
            }
            fields.push(row);
        }

        return Chessboard {
            board: quad,
            pieces: vec!(),
            piece_factory,
            tx: chessboard_texture,
            field_size: field_size as u32,
            board_size: board_size as u32,
            position,
            prev_mouse_pos: (0.0, 0.0),
            fields,
            dragger_piece: None,
        };
    }

    fn get_field_position(&self, field: &Field) -> (i32, i32, i32) {
        (
            field.logic.col as i32 * self.field_size as i32 + self.position.0,
            field.logic.row as i32 * self.field_size as i32 + self.position.1,
            0
        )
    }

    pub fn init_pieces(&mut self, pieces_sheet: &'a Texture) {
        let piece_size = (self.field_size as i32, self.field_size as i32);
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::WHITE, pieces_sheet, self.get_field_by_name("A1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::WHITE, pieces_sheet, self.get_field_by_name("B1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::WHITE, pieces_sheet, self.get_field_by_name("C1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::QUEEN, Side::WHITE, pieces_sheet, self.get_field_by_name("D1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KING, Side::WHITE, pieces_sheet, self.get_field_by_name("E1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::WHITE, pieces_sheet, self.get_field_by_name("F1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::WHITE, pieces_sheet, self.get_field_by_name("G1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::WHITE, pieces_sheet, self.get_field_by_name("H1"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("A2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("B2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("C2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("D2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("E2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("F2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("G2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, pieces_sheet, self.get_field_by_name("H2"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::BLACK, pieces_sheet, self.get_field_by_name("A8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::BLACK, pieces_sheet, self.get_field_by_name("B8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::BLACK, pieces_sheet, self.get_field_by_name("C8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::QUEEN, Side::BLACK, pieces_sheet, self.get_field_by_name("D8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KING, Side::BLACK, pieces_sheet, self.get_field_by_name("E8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::BLACK, pieces_sheet, self.get_field_by_name("F8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::BLACK, pieces_sheet, self.get_field_by_name("G8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::BLACK, pieces_sheet, self.get_field_by_name("H8"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("A7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("B7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("C7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("D7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("E7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("F7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("G7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, pieces_sheet, self.get_field_by_name("H7"), piece_size));
    }

    // todo holy shit we operate in both coordinate systems at the same time...
    // todo even context is needed here to translate them...
    // todo horror
    pub fn handle_event(&mut self, event: &sdl2::event::Event, mouse_coords_px: &(i32, i32), mouse_coords_opengl: &(f32, f32), context: &OpenglContext) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                for (i, obj) in self.pieces.iter_mut().enumerate() {
                    if obj.is_mouse_over(mouse_coords_opengl) {
                        self.dragger_piece = Some(i);
                        obj.handle_start_drag();
                    }
                }
                if self.dragger_piece != None {
                    let allowed_fields = &mut self.pieces[self.dragger_piece.unwrap()].logic.get_all_allowed_moves(&ChessboardState { occupied_fields: HashMap::new() });
                    allowed_fields.iter_mut().for_each(|allowed_field| { self.fields[allowed_field.row as usize][allowed_field.col as usize].is_possible_move = true; })
                }
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                match self.get_field_by_point(mouse_coords_px) {
                    None => {
                        if self.dragger_piece != None {
                            self.pieces[self.dragger_piece.unwrap()].return_to_initial_pos();
                        }
                    }
                    Some(field) => {
                        // todo: i dont know how to do this without two clones, thanks rust, I'm safe :D
                        if self.dragger_piece != None {
                            let field_data = field.logic.clone();
                            let pos = field.get_position_3d();
                            self.pieces[self.dragger_piece.unwrap()]
                                .handle_drop(
                                    context,
                                    field_data.clone(),
                                    pos,
                                    &ChessboardState { occupied_fields: HashMap::new() }
                                );
                        }
                    }
                }
                self.clear_allowed_fields();
                self.dragger_piece = None;
            }
            sdl2::event::Event::MouseMotion { .. } => {
                let drag_offset = &(
                    (mouse_coords_opengl.0 - self.prev_mouse_pos.0) as f32,
                    (mouse_coords_opengl.1 - self.prev_mouse_pos.1) as f32
                );

                if self.dragger_piece != None {
                    self.pieces[self.dragger_piece.unwrap()].handle_drag_pointer_move(drag_offset);
                }
            }

            _ => {}
        }
        self.prev_mouse_pos = mouse_coords_opengl.clone()
    }

    fn get_field_by_name(&self, name: &str) -> &Field {
        // hella inefficient, we just know don't need to check everywhere
        for row in self.fields.iter() {
            for f in row {
                if f.logic.name == name {
                    return f;
                }
            }
        }
        panic!("Asking for non existent field {}", name)
    }


    // could also move it to field.contains() or something
    fn get_field_by_point(&self, point: &(i32, i32)) -> Option<&Field> {
        return match self.get_field_coords_by_point(point) {
            None => None,
            Some(coords) => {
                let field = &self.fields[coords.0][coords.1];
                Some(field)
            }
        };
    }

    // todo: should be get field by point...
    fn get_field_coords_by_point(&self, point: &(i32, i32)) -> Option<(usize, usize)> {
        if point.0 < self.position.0 ||
            point.0 as i32 > self.position.0 + self.board_size as i32 ||
            point.1 < self.position.1 ||
            point.1 as i32 > self.position.1 + self.board_size as i32 {
            return None;
        }
        return Some(
            (
                7 - ((point.1 as i32 - self.position.1) / self.field_size as i32) as usize,
                ((point.0 as i32 - self.position.0) / self.field_size as i32) as usize,
            )
        );
    }

    fn clear_allowed_fields(&mut self) {
        self.fields.iter_mut().for_each(|row| row.iter_mut().for_each(|field| field.is_possible_move = false))
    }
}

impl<'a> Drawable for Chessboard<'a> {
    fn render(&self) {
        self.board.render();
        self.fields.iter().for_each(|row| row.iter().for_each(|field| field.render()));
        self.pieces.iter().for_each(|piece| { piece.render() });
    }
}

pub struct ChessboardState {
    occupied_fields: HashMap<FieldLogic, PieceLogic>,
}


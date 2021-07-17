use std::borrow::Borrow;
use std::path::Path;

use sdl2::keyboard::Keycode;

use crate::maths::quadrangle::Quadrangle;
use crate::maths::segment::Segment;
use crate::maths::triangle::{Drawable, Triangle};
use crate::maths::vertex;
use crate::maths::vertex::VertexTextured;
use crate::mouse_drag_controller::{MouseDragController, Draggable};
use crate::opengl_context::OpenglContext;
use crate::resources::Resources;
use crate::texture::{SpriteCoords, Texture};
use crate::vertex::VertexShaderDataSetter;

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod opengl_context;
pub mod texture;

mod maths;
mod mouse_drag_controller;

fn main() {
    let context = OpenglContext::init();
    let mut event_pump = context.sdl.event_pump().unwrap();

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
    let tx_shader_program = render_gl::Program::from_res(&res, "shaders/texture").unwrap();

    let chessboard_data = res.load_image("textures/chessboard.png");
    let chessboard_texture = Texture::from_image(chessboard_data);
    let mut chessboard = Chessboard::new(&chessboard_texture, &res, &context, &tx_shader_program);
    let pieces = res.load_image("textures/pieces.png");
    let pieces_texture = Texture::spritesheet_from_image(pieces, 2, 6);
    chessboard.init_pieces(&pieces_texture);

    let triangle2 = Triangle::new(
        [
            vertex::VertexColored { pos: (-1.0, -0.9, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
            vertex::VertexColored { pos: (-0.7, -0.9, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },
            vertex::VertexColored { pos: (-0.85, -0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
        ],
        [0, 1, 2],
        &shader_program,
        None,
    );

    let segment = Segment::new(
        [
            vertex::VertexColored { pos: (0.0, 0.1, 0.0).into(), clr: (0.0, 0.0, 0.0).into() },
            vertex::VertexColored { pos: (0.1, -0.1, 0.0).into(), clr: (0.0, 0.0, 0.0).into() },
        ],
        [0, 1],
        &shader_program,
    );

    let mut renderer = renderer::Renderer::new(&context);

    'main: loop {
        let window_mouse_coords = &(event_pump.mouse_state().x(), event_pump.mouse_state().y());
        let mouse_opengl_coords = context.sdl_window_to_opengl_space(window_mouse_coords);

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    renderer.resize_viewport(w, h);
                }
                sdl2::event::Event::KeyDown {
                    keycode,
                    ..
                } => {}
                _ => {}
            }

            chessboard.handle_event(&event, &mouse_opengl_coords)
        }

        renderer.render(&[
            // &triangle2,
            // &player,
            // &quad,
            // &quad2,
            // &segment,
            // &piece
            &chessboard
        ]);
    }
}


struct Player<'a, T: VertexShaderDataSetter> {
    pub triangle: Triangle<'a, T>,
}

impl<'a, T: VertexShaderDataSetter> Player<'a, T> {
    fn new(triangle: Triangle<T>) -> Player<T> {
        Player { triangle }
    }

    fn handle_input(&mut self, keycode: Keycode) {
        let move_speed: f32 = 0.1;
        match keycode {
            sdl2::keyboard::Keycode::Left => {
                self.triangle.move_by(-move_speed, 0.0, 0.0)
            }
            sdl2::keyboard::Keycode::Right => {
                self.triangle.move_by(move_speed, 0.0, 0.0)
            }
            sdl2::keyboard::Keycode::Up => {
                self.triangle.move_by(0.0, move_speed, 0.0)
            }
            sdl2::keyboard::Keycode::Down => {
                self.triangle.move_by(0.0, -move_speed, 0.0)
            }
            _ => {}
        }
    }
}

impl<'a, T: VertexShaderDataSetter> Drawable for Player<'a, T> {
    fn render(&self) {
        self.triangle.render();
    }
}


// todo: this should be encapsulated into shapes
fn create_rect_coords_in_opengl_space(
    context: &OpenglContext, pos: (i32, i32, i32), size: (i32, i32), sprite_coords: &SpriteCoords) -> [VertexTextured; 4] {
    return [
        vertex::VertexTextured { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1 + size.1, pos.2)).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.top_right.into() },
        vertex::VertexTextured { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1, pos.2)).into(), clr: (1.0, 1.0, 0.0, 1.0).into(), tx_coords: sprite_coords.bottom_right.into() },
        vertex::VertexTextured { pos: context.engine_to_opengl_space(&(pos.0, pos.1, pos.2)).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.bottom_left.into() },
        vertex::VertexTextured { pos: context.engine_to_opengl_space(&(pos.0, pos.1 + size.1, pos.2)).into(), clr: (1.0, 1.0, 1.0, 1.0).into(), tx_coords: sprite_coords.top_left.into() },
    ];
}


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

    pub fn init_piece(&self, piece_type: PieceType, pieces_sheet: &'a Texture) -> Piece<'a> {
        let quad = Quadrangle::new(
            create_rect_coords_in_opengl_space(
                &self.opengl_context,
                (50, 100, 0),
                (300, 300),
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
}

impl<'a> Drawable for Chessboard<'a> {
    fn render(&self) {
        self.board.render();
        self.pieces.iter().for_each(|piece| { piece.render() })
    }
}

impl<'a> Chessboard<'a> {
    pub fn new(chessboard_texture: &'a Texture, res: &Resources, opengl_context: &'a OpenglContext, shader: &'a render_gl::Program) -> Chessboard<'a> {
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
        };
    }

    pub fn init_pieces(&mut self, pieces_sheet: &'a Texture) {
        let piece = self.piece_factory.init_piece(PieceType::PAWN, pieces_sheet);
        self.pieces.push(piece);
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event, mouse_pos: &(f32, f32)) {
        self.mouse_drag_controller.handle_event(event, mouse_pos, &mut self.pieces)
    }
}

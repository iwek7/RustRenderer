use std::path::Path;

use sdl2::keyboard::Keycode;

use crate::maths::segment::Segment;
use crate::maths::triangle::{Drawable, Triangle};
use crate::maths::vertex;
use crate::maths::vertex::VertexTextured;
use crate::opengl_context::OpenglContext;
use crate::resources::Resources;
use crate::texture::{SpriteCoords, Texture};
use crate::vertex::VertexShaderDataSetter;
use crate::chess::chessboard::Chessboard;

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod opengl_context;
pub mod texture;

mod maths;
mod chess;

fn main() {
    let context = OpenglContext::init();
    let mut event_pump = context.sdl.event_pump().unwrap();

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
    let tx_shader_program = render_gl::Program::from_res(&res, "shaders/texture").unwrap();

    let chessboard_data = res.load_image("textures/chessboard.png");
    let chessboard_texture = Texture::from_image(chessboard_data);
    let mut chessboard = Chessboard::new(&chessboard_texture, &context, &tx_shader_program);
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
        let mouse_coords_px = &(event_pump.mouse_state().x(), event_pump.mouse_state().y());
        let mouse_opengl_coords = context.sdl_window_to_opengl_space(mouse_coords_px);

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

            chessboard.handle_event(&event, mouse_coords_px, &mouse_opengl_coords, &context)
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

// todo: should not be here
pub struct SpriteSheet {
    sprite_sheet: Texture,
}


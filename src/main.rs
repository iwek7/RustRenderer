use std::ffi::CString;
use std::path::Path;

use sdl2::keyboard::Keycode;

use render_gl::buffer;
use render_gl::data;

use crate::opengl_context::OpenglContext;
use crate::resources::Resources;
use crate::triangle::{ObjectRender, Triangle};

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod vertex;
pub mod triangle;
pub mod opengl_context;

fn main() {
    let context = OpenglContext::init();
    let mut event_pump = context.sdl.event_pump().unwrap();

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
    shader_program.set_used();

    let triangle = Triangle::new(
        vertex::Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() }, // bottom right
        vertex::Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
        vertex::Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
        &shader_program);
    let mut player = Player::new(triangle);

    let triangle2 = Triangle::new(
        vertex::Vertex { pos: (-1.0, -0.9, 0.0).into(), clr: (1.0, 0.0, 0.0).into() }, // bottom right
        vertex::Vertex { pos: (-0.7, -0.9, 0.0).into(), clr: (0.0, 1.0, 0.0).into() }, // bottom left
        vertex::Vertex { pos: (-0.85, -0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
        &shader_program);

    let mut renderer = renderer::Renderer::new(context);

    'main: loop {
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
                } => {
                    player.handle_input(keycode.unwrap());
                }
                _ => {}
            }
        }

        renderer.render(&[&triangle2, &player]);
    }
}


struct Player<'a> {
    pub triangle: Triangle<'a>,
}

impl<'a> Player<'a> {
    fn new(triangle: Triangle) -> Player {
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

impl<'a> ObjectRender for Player<'a> {
    fn render(&self) {
        self.triangle.render();
    }
}

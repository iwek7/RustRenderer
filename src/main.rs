use std::ops::DerefMut;
use std::path::Path;

use sdl2::keyboard::Keycode;

use crate::opengl_context::OpenglContext;
use crate::resources::Resources;
use crate::shape_quadrangle::Quadrangle;
use crate::shape_triangle::{Area, Drawable, Triangle};
use crate::texture::Texture;
use crate::vertex::VertexShaderDataSetter;

pub mod render_gl;
pub mod resources;
pub mod renderer;
pub mod vertex;
pub mod shape_triangle;
pub mod opengl_context;
pub mod texture;
mod shape_quadrangle;

fn main() {
    let context = OpenglContext::init();
    let mut event_pump = context.sdl.event_pump().unwrap();

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let chessboard_data = res.load_image("textures/chessboard.png");
    let texture = Texture::from_image(chessboard_data);

    let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
    let tx_shader_program = render_gl::Program::from_res(&res, "shaders/texture").unwrap();

    let mut mouse_drag_controller = MouseDragController::new();

    let triangle = Triangle::new(
        [
            vertex::VertexColored { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
            vertex::VertexColored { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0).into() },
            vertex::VertexColored { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0).into() },
        ],
        [0, 1, 2],
        &shader_program,
        None,
    );
    let mut player = Player::new(triangle);

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

    let quad = Quadrangle::new(
        [
            vertex::VertexTextured { pos: (0.7, 0.7, 0.0).into(), clr: (1.0, 0.0, 0.0).into(), tx_coords: (1.0, 1.0).into() },
            vertex::VertexTextured { pos: (0.7, -0.7, 0.0).into(), clr: (1.0, 1.0, 0.0).into(), tx_coords: (0.0, 1.0).into() },
            vertex::VertexTextured { pos: (-0.7, -0.7, 0.0).into(), clr: (1.0, 0.0, 1.0).into(), tx_coords: (0.0, 0.0).into() },
            vertex::VertexTextured { pos: (-0.7, 0.7, 0.0).into(), clr: (0.0, 1.0, 1.0).into(), tx_coords: (1.0, 0.0).into() },
        ],
        [0, 1, 3, 1, 2, 3],
        &tx_shader_program,
        Some(texture),
    );

    let mut quad2 = Quadrangle::new(
        [
            vertex::VertexColored { pos: (0.1, 0.1, 0.0).into(), clr: (1.0, 0.0, 0.0).into() },
            vertex::VertexColored { pos: (0.1, -0.1, 0.0).into(), clr: (1.0, 1.0, 0.0).into() },
            vertex::VertexColored { pos: (-0.1, -0.1, 0.0).into(), clr: (1.0, 0.0, 1.0).into() },
            vertex::VertexColored { pos: (-0.1, 0.1, 0.0).into(), clr: (0.0, 1.0, 1.0).into() }
        ],
        [0, 1, 3, 1, 2, 3],
        &shader_program,
        None,
    );

    let mut renderer = renderer::Renderer::new(&context);

    'main: loop {
        let window_mouse_coords = &(event_pump.mouse_state().x(), event_pump.mouse_state().y());
        let opengl_coords_mouse_coords = context.window_to_opengl_space(window_mouse_coords);

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
            mouse_drag_controller.handle_event(&event, &opengl_coords_mouse_coords, &mut [&mut quad2])
        }

        renderer.render(&[
            &triangle2,
            &player,
            &quad,
            &quad2
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

struct MouseDragController<> {
    prev_mouse_pos: (f32, f32),
}

impl<> MouseDragController<> {
    pub fn new() -> MouseDragController<> {
        return MouseDragController {
            prev_mouse_pos: (0.0, 0.0)
        };
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event,
                        mouse_pos: &(f32, f32),
                        objects: &mut [&mut dyn Draggable]) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                for obj in objects.iter_mut() {
                    if obj.is_mouse_over(mouse_pos) {
                        obj.handle_start_drag()
                    }
                }
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                objects.iter_mut().for_each(|it| { it.handle_drop() })
            }
            sdl2::event::Event::MouseMotion { .. } => {
                objects.iter_mut()
                    .for_each(|it| {
                        it.handle_drag_pointer_move(&(
                            mouse_pos.0 - self.prev_mouse_pos.0,
                            mouse_pos.1 - self.prev_mouse_pos.1
                        ))
                    });
            }
            _ => {}
        }
        self.prev_mouse_pos = mouse_pos.clone()
    }
}

trait Draggable {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool;
    fn handle_start_drag(&mut self);
    fn handle_drop(&mut self);
    fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32));
}

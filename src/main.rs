use std::path::Path;

use crate::chess::chess_game::ChessGame;
use crate::maths::vertex;
use crate::maths::vertex::{VertexColored, VertexTextured};
use crate::opengl_context::OpenglContext;
use crate::resources::Resources;
use crate::texture::{SpriteCoords, Texture};

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

    let pieces = res.load_image("textures/pieces.png");
    let pieces_texture = Texture::spritesheet_from_image(pieces, 2, 6);

    let black_win_banner_data = res.load_image("textures/black_win_banner.png");
    let black_win_banner_texture = Texture::from_image(black_win_banner_data);

    let white_win_banner_data = res.load_image("textures/white_win_banner.png");
    let white_win_banner_texture = Texture::from_image(white_win_banner_data);

    let mut chess_game = ChessGame::initialize(&chessboard_texture,
                                               &pieces_texture,
                                               &context,
                                               &tx_shader_program,
                                               &shader_program,
                                               &white_win_banner_texture,
                                               &black_win_banner_texture);

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

            chess_game.handle_event(&event, mouse_coords_px, &mouse_opengl_coords, &context)
        }

        renderer.render(&[
            &chess_game
        ]);
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

// todo: size should be u32
fn create_rect_coords_in_opengl_space_colored(
    context: &OpenglContext, pos: (i32, i32, i32), size: (i32, i32), clr: (f32, f32, f32, f32)) -> [VertexColored; 4] {
    return [
        vertex::VertexColored { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1 + size.1, pos.2)).into(), clr: clr.into() },
        vertex::VertexColored { pos: context.engine_to_opengl_space(&(pos.0 + size.0, pos.1, pos.2)).into(), clr: clr.into() },
        vertex::VertexColored { pos: context.engine_to_opengl_space(&(pos.0, pos.1, pos.2)).into(), clr: clr.into() },
        vertex::VertexColored { pos: context.engine_to_opengl_space(&(pos.0, pos.1 + size.1, pos.2)).into(), clr: clr.into() },
    ];
}


// todo: should not be here
pub struct SpriteSheet {
    sprite_sheet: Texture,
}


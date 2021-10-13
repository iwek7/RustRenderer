use std::path::Path;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use crate::chess::chessboard::Chessboard;
use crate::chess::infrastructure::Side;
use crate::create_rect_coords_deprecated;
use crate::engine::api::camera::CameraGameObject;
use crate::engine::api::coordinate_system::CoordinateSystem;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::maths::point::Point;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::engine::api::resource_manager::ResourceManager;
use crate::engine::api::texture::{Texture, TextureFilterType, TextureParams};
use crate::engine::game_controller::{CameraConfig, GameController};
use crate::engine::render_gl;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::resources::ResourceLoader;
use crate::engine::opengl_context::OpenglContext;

pub struct ChessGame {
    chessboard: Chessboard,
    black_win_banner: Quadrangle<TexturedVertexData>,
    white_win_banner: Quadrangle<TexturedVertexData>,
}

impl ChessGame {
    pub fn new(resource_manager: Rc<ResourceManager>) -> ChessGame {
        let mut chessboard = Chessboard::new(Rc::clone(&resource_manager));
        chessboard.init_pieces(Rc::clone(&resource_manager));

        let texture_shader = resource_manager.fetch_shader_program("chess/shaders/texture");

        let white_win_banner = ChessGame::create_win_banner(
            resource_manager.fetch_texture("chess/textures/white_win_banner.png"),
            Rc::clone(&texture_shader),
        );

        let black_win_banner = ChessGame::create_win_banner(
            resource_manager.fetch_texture("chess/textures/black_win_banner.png"),
            Rc::clone(&texture_shader),
        );

        ChessGame {
            chessboard,
            black_win_banner,
            white_win_banner,
        }
    }

    fn create_win_banner(tx: Rc<Texture>, shader: Rc<render_gl::ShaderProgram>) -> Quadrangle<TexturedVertexData> {
        Quadrangle::new(
            create_rect_coords_deprecated((200.0, 100.0, 0.0), (512.0, 512.0),
                                          &tx.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            shader,
            Some(Rc::clone(&tx)),
        )
    }
}

impl Drawable for ChessGame {
    fn render(&self, render_util: &RenderUtil) {
        match self.chessboard.get_winner() {
            None => { self.chessboard.render(render_util) }
            Some(winning_side) => {
                self.chessboard.render(render_util);
                match winning_side {
                    Side::BLACK => { self.black_win_banner.render(render_util) }
                    Side::WHITE => { self.white_win_banner.render(render_util) }
                }
            }
        }
    }

    fn handle_event(&mut self, event: &Event, context: &OpenglContext, update_context: &UpdateContext) {
        match context.sdl_space_to_world_space_at_z0(update_context.get_sdl_mouse_position(), &update_context.get_camera_config()) {
            None => {}
            Some(world_mouse_position) => {
                match event {
                    sdl2::event::Event::MouseButtonDown { .. } => {
                        self.chessboard.handle_start_piece_dragging_attempt(&world_mouse_position);
                    }

                    sdl2::event::Event::MouseButtonUp { .. } => {
                        self.chessboard.handle_piece_drop_attempt(&world_mouse_position, Rc::clone(&update_context.resource_manager));
                    }

                    sdl2::event::Event::MouseMotion { .. } => {
                        self.chessboard.handle_piece_dragging_attempt(&world_mouse_position);
                    }

                    _ => {}
                }
            }
        }
    }
}


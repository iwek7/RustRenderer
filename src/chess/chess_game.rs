use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::chess::chessboard::Chessboard;
use crate::chess::infrastructure::Side;
use crate::engine::api::drawable::{Drawable, UpdateContext};
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::vertex::TexturedVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::api::texture::Texture;
use crate::engine::opengl_context::OpenglContext;
use crate::engine::rendering;
use crate::engine::rendering::material::Material;

pub struct ChessGame {
    chessboard: Chessboard,
    black_win_banner: Rectangle<TexturedVertexDataLayout>,
    white_win_banner: Rectangle<TexturedVertexDataLayout>,
}

impl ChessGame {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> ChessGame {
        let res_manager = engine_utilities.get_resource_manager();

        let mut chessboard = Chessboard::new(Rc::clone(&res_manager));
        chessboard.init_pieces(Rc::clone(&res_manager));

        let texture_material = res_manager.fetch_shader_material("chess/shaders/texture");

        let white_win_banner = ChessGame::create_win_banner(
            res_manager.fetch_texture("chess/textures/white_win_banner.png"),
            texture_material.clone(),
        );

        let black_win_banner = ChessGame::create_win_banner(
            res_manager.fetch_texture("chess/textures/black_win_banner.png"),
            texture_material,
        );

        ChessGame {
            chessboard,
            black_win_banner,
            white_win_banner,
        }
    }

    fn create_win_banner(tx: Rc<Texture>, material: Material) -> Rectangle<TexturedVertexDataLayout> {
        Rectangle::new_textured(
            &glam::vec3(2.0, 2.0, 0.0),
            &glam::vec2(4., 4.),
            material,
            tx,
        )
    }
}

impl Drawable for ChessGame {
    fn render(&mut self, render_util: &RenderUtil) {
        match self.chessboard.get_winner().clone() {
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
                        self.chessboard.handle_piece_drop_attempt(&world_mouse_position, Rc::clone(&update_context.get_engine_utilities().get_resource_manager()));
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


use std::path::Path;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use crate::{create_rect_coords, render_gl};
use crate::api::camera::CameraGameObject;
use crate::api::coordinate_system::CoordinateSystem;
use crate::api::drawable::{Drawable, UpdateContext};
use crate::api::resource_manager::ResourceManager;
use crate::chess::chessboard::Chessboard;
use crate::chess::infrastructure::Side;
use crate::engine::game_controller::{CameraConfig, GameController};
use crate::maths::point::Point;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::opengl_context::OpenglContext;
use crate::renderer::{Renderer, RenderUtil};
use crate::resources::ResourceLoader;
use crate::texture::{Texture, TextureFilterType, TextureParams};

pub struct ChessGame {
    chessboard: Chessboard,
    black_win_banner: Quadrangle<TexturedVertexData>,
    white_win_banner: Quadrangle<TexturedVertexData>,
}

impl ChessGame {
    // todo wrap this sdl event pump in adapter
    pub fn new(resource_manager: Rc<ResourceManager>) -> ChessGame {

        // let color_shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
        // let tx_shader_program = render_gl::Program::from_res(&res, "shaders/texture").unwrap();
        //
        // let chessboard_data = res.load_image("textures/chessboard.png");
        // let chessboard_texture = Texture::from_image(chessboard_data, TextureParams::new());
        //
        // let pieces = res.load_image("textures/pieces.png");
        // let pieces_texture = Texture::spritesheet_from_image(pieces, 2, 6, TextureParams::new());

        let banner_tx_params = TextureParams::new()
            .with_mag_filter(TextureFilterType::NEAREST)
            .with_min_filter(TextureFilterType::NEAREST);

        // let black_win_banner_data = res.load_image("textures/black_win_banner.png");
        // let black_win_banner_texture = Texture::from_image(black_win_banner_data, banner_tx_params.clone());
        //
        // let white_win_banner_data = res.load_image("textures/white_win_banner.png");
        // let white_win_banner_texture = Texture::from_image(white_win_banner_data, banner_tx_params);

        let mut chessboard = Chessboard::new(Rc::clone(&resource_manager));
        chessboard.init_pieces(Rc::clone(&resource_manager));

        let texture_shader = resource_manager.fetch_shader_program("shaders/texture");

        let white_win_banner = ChessGame::create_win_banner(
            resource_manager.fetch_texture("textures/white_win_banner.png"),
            Rc::clone(&texture_shader),
        );

        let black_win_banner = ChessGame::create_win_banner(
            resource_manager.fetch_texture("textures/black_win_banner.png"),
            Rc::clone(&texture_shader),
        );

        ChessGame {
            chessboard,
            black_win_banner,
            white_win_banner,
        }
    }

    fn create_win_banner(tx: Rc<Texture>, shader: Rc<render_gl::Program>) -> Quadrangle<TexturedVertexData> {
        Quadrangle::new(
            create_rect_coords((200.0, 100.0, 0.0), (512.0, 512.0),
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


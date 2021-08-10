use std::path::Path;

use sdl2::EventPump;

use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::api::camera::CameraGameObject;
use crate::chess::chessboard::Chessboard;
use crate::chess::infrastructure::Side;
use crate::chess::resource_manager::ResourceManager;
use crate::engine::game_controller::{CameraConfig, GameController};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::TexturedVertexData;
use crate::opengl_context::OpenglContext;
use crate::renderer::{Renderer, RenderUtil};
use crate::resources::Resources;
use crate::texture::{Texture, TextureFilterType, TextureParams};

pub struct ChessGame<'a> {
    chessboard: Chessboard<'a>,
    camera: CameraGameObject,
    black_win_banner: Quadrangle<'a, TexturedVertexData>,
    white_win_banner: Quadrangle<'a, TexturedVertexData>,
}

impl<'a> ChessGame<'a> {
    // todo wrap this sdl event pump in adapter
    pub fn play(renderer: &mut Renderer, event_pump: &mut EventPump, context: &OpenglContext) {
        let res = Resources::from_relative_exe_path(Path::new("assets/chess")).unwrap();

        let shader_program = render_gl::Program::from_res(&res, "shaders/triangle").unwrap();
        let tx_shader_program = render_gl::Program::from_res(&res, "shaders/texture").unwrap();

        let chessboard_data = res.load_image("textures/chessboard.png");
        let chessboard_texture = Texture::from_image(chessboard_data, TextureParams::new());

        let pieces = res.load_image("textures/pieces.png");
        let pieces_texture = Texture::spritesheet_from_image(pieces, 2, 6, TextureParams::new());

        let banner_tx_params = TextureParams::new()
            .with_mag_filter(TextureFilterType::NEAREST)
            .with_min_filter(TextureFilterType::NEAREST);

        let black_win_banner_data = res.load_image("textures/black_win_banner.png");
        let black_win_banner_texture = Texture::from_image(black_win_banner_data, banner_tx_params.clone());

        let white_win_banner_data = res.load_image("textures/white_win_banner.png");
        let white_win_banner_texture = Texture::from_image(white_win_banner_data, banner_tx_params);

        let mut chess_game = ChessGame::initialize(&chessboard_texture,
                                                   &pieces_texture,
                                                   &context,
                                                   &tx_shader_program,
                                                   &shader_program,
                                                   &white_win_banner_texture,
                                                   &black_win_banner_texture);

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

            renderer.render(
                &[&chess_game],
                &chess_game.get_camera_config(),
            );
        }
    }

    fn initialize(chessboard_texture: &'a Texture,
                  pieces_texture: &'a Texture,
                  opengl_context: &'a OpenglContext,
                  chessboard_shader: &'a render_gl::Program,
                  possible_move_shader: &'a render_gl::Program,
                  white_win_banner_texture: &'a Texture,
                  black_win_banner_texture: &'a Texture,
    ) -> ChessGame<'a> {
        let resource_manager = ResourceManager::new(chessboard_texture, chessboard_shader, possible_move_shader, pieces_texture);
        let mut chessboard = Chessboard::new(&opengl_context, resource_manager);

        chessboard.init_pieces();

        let white_win_banner = ChessGame::create_win_banner(white_win_banner_texture, chessboard_shader, opengl_context);
        let black_win_banner = ChessGame::create_win_banner(black_win_banner_texture, chessboard_shader, opengl_context);

        ChessGame {
            chessboard,
            black_win_banner,
            white_win_banner,
            camera: CameraGameObject::new(),
        }
    }


    fn create_win_banner(tx: &'a Texture, shader: &'a render_gl::Program, opengl_context: &'a OpenglContext) -> Quadrangle<'a, TexturedVertexData> {
        Quadrangle::new(
            create_rect_coords_in_opengl_space(&opengl_context, (200, 100, 0), (512, 512),
                                               &tx.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            &shader,
            Some(&tx),
        )
    }

    fn handle_event(&mut self, event: &sdl2::event::Event, mouse_coords_px: &(i32, i32), mouse_coords_opengl: &(f32, f32), context: &OpenglContext) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                self.chessboard.handle_start_piece_dragging_attempt(mouse_coords_opengl);
            }

            sdl2::event::Event::MouseButtonUp { .. } => {
                self.chessboard.handle_piece_drop_attempt(mouse_coords_px, mouse_coords_opengl, context);
            }

            sdl2::event::Event::MouseMotion { .. } => {
                self.chessboard.handle_piece_dragging_attempt(mouse_coords_opengl);
            }
            _ => {}
        }
    }
}

impl<'a> Drawable for ChessGame<'a> {
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
}

impl<'a> GameController for ChessGame<'a> {
    fn get_camera_config(&self) -> CameraConfig {
        CameraConfig::new()
    }
}
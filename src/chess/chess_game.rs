use std::path::Path;

use glam::Vec3;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use crate::{create_rect_coords, create_rect_coords_colored, render_gl};
use crate::api::camera::CameraGameObject;
use crate::chess::chessboard::Chessboard;
use crate::chess::infrastructure::Side;
use crate::chess::resource_manager::ResourceManager;
use crate::engine::game_controller::{CameraConfig, GameController};
use crate::maths::point::Point;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::segment::Segment;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::{ColoredVertexData, TexturedVertexData};
use crate::opengl_context::OpenglContext;
use crate::renderer::{Renderer, RenderUtil};
use crate::resources::Resources;
use crate::texture::{Texture, TextureFilterType, TextureParams};
use crate::api::coordinate_system::CoordinateSystem;

const CAMERA_SPEED: f32 = 0.3;

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

        let mut test_mouse_point = Point::new(
            [ColoredVertexData { pos: (0.0, -0.0, 0.0).into(), clr: (0.0, 0.0, 0.0, 1.0).into() }, ],
            &shader_program,
        );

        let mut chess_game = ChessGame::initialize(&chessboard_texture,
                                                   &pieces_texture,
                                                   &tx_shader_program,
                                                   &shader_program,
                                                   &white_win_banner_texture,
                                                   &black_win_banner_texture);

        let mut coordinate_system = CoordinateSystem::new(&shader_program);

        'main: loop {
            let sdl_pos = (event_pump.mouse_state().x().clone(), event_pump.mouse_state().y().clone());

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

                match context.sdl_space_to_world_space_at_z0(&sdl_pos, &chess_game.get_camera_config()) {
                    None => {}
                    Some(world_mouse_position) => {
                        test_mouse_point.move_to(&world_mouse_position);
                        chess_game.handle_event(&event, &world_mouse_position, &context)
                    }
                }
            }

            renderer.render(
                &[&chess_game, &coordinate_system, &test_mouse_point],
                &chess_game.get_camera_config(),
            );
        }
    }

    fn initialize(chessboard_texture: &'a Texture,
                  pieces_texture: &'a Texture,
                  chessboard_shader: &'a render_gl::Program,
                  possible_move_shader: &'a render_gl::Program,
                  white_win_banner_texture: &'a Texture,
                  black_win_banner_texture: &'a Texture,
    ) -> ChessGame<'a> {
        let resource_manager = ResourceManager::new(chessboard_texture, chessboard_shader, possible_move_shader, pieces_texture);
        let mut chessboard = Chessboard::new(resource_manager);

        chessboard.init_pieces();

        let white_win_banner = ChessGame::create_win_banner(white_win_banner_texture, chessboard_shader);
        let black_win_banner = ChessGame::create_win_banner(black_win_banner_texture, chessboard_shader);

        ChessGame {
            chessboard,
            black_win_banner,
            white_win_banner,
            camera: CameraGameObject::new(glam::vec3(0.0, 0.0, 10.0)),
        }
    }


    fn create_win_banner(tx: &'a Texture, shader: &'a render_gl::Program) -> Quadrangle<'a, TexturedVertexData> {
        Quadrangle::new(
            create_rect_coords((200.0, 100.0, 0.0), (512.0, 512.0),
                               &tx.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            &shader,
            Some(&tx),
        )
    }

    fn handle_event(&mut self, event: &sdl2::event::Event, world_mouse_position: &glam::Vec3, context: &OpenglContext) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                self.chessboard.handle_start_piece_dragging_attempt(world_mouse_position);
            }

            sdl2::event::Event::MouseButtonUp { .. } => {
                self.chessboard.handle_piece_drop_attempt(world_mouse_position);
            }

            sdl2::event::Event::MouseMotion { .. } => {
                self.chessboard.handle_piece_dragging_attempt(world_mouse_position);
            }

            sdl2::event::Event::KeyDown { keycode, .. } => {
                match keycode.unwrap() {
                    Keycode::Left => {
                        self.camera.move_by(glam::Vec3::new(-CAMERA_SPEED, 0.0, 0.0))
                    }
                    Keycode::Right => {
                        self.camera.move_by(glam::Vec3::new(CAMERA_SPEED, 0.0, 0.0))
                    }
                    Keycode::Down => {
                        self.camera.move_by(glam::Vec3::new(0.0, -CAMERA_SPEED, 0.0))
                    }
                    Keycode::Up => {
                        self.camera.move_by(glam::Vec3::new(0.0, CAMERA_SPEED, 0.0))
                    }
                    _ => {}
                }
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
        self.camera.get_current_config()
    }
}
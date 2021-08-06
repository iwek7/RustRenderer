use crate::{create_rect_coords_in_opengl_space, render_gl};
use crate::chess::chessboard::Chessboard;
use crate::chess::infrastructure::Side;
use crate::chess::resource_manager::ResourceManager;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexTextured;
use crate::opengl_context::OpenglContext;
use crate::texture::Texture;

pub struct ChessGame<'a> {
    chessboard: Chessboard<'a>,
    black_win_banner: Quadrangle<'a, VertexTextured>,
    white_win_banner: Quadrangle<'a, VertexTextured>,
}

impl<'a> ChessGame<'a> {
    pub fn initialize(chessboard_texture: &'a Texture,
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
        }
    }

    fn create_win_banner(tx: &'a Texture, shader: &'a render_gl::Program, opengl_context: &'a OpenglContext) -> Quadrangle<'a, VertexTextured> {
        Quadrangle::new(
            create_rect_coords_in_opengl_space(&opengl_context, (200, 100, 0), (512, 512),
                                               &tx.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            &shader,
            Some(&tx),
        )
    }

    pub fn handle_event(&mut self, event: &sdl2::event::Event, mouse_coords_px: &(i32, i32), mouse_coords_opengl: &(f32, f32), context: &OpenglContext) {
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
    fn render(&self) {
        match self.chessboard.get_winner() {
            None => { self.chessboard.render() }
            Some(winning_side) => {
                self.chessboard.render();
                match winning_side {
                    Side::BLACK => { self.black_win_banner.render() }
                    Side::WHITE => { self.white_win_banner.render() }
                }
            }
        }
    }
}
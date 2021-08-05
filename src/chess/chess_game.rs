use crate::chess::chessboard::Chessboard;
use crate::maths::triangle::Drawable;
use crate::opengl_context::OpenglContext;
use crate::render_gl;
use crate::texture::Texture;
use crate::chess::resource_manager::ResourceManager;

pub struct ChessGame<'a> {
    chessboard: Chessboard<'a>
}

impl<'a> ChessGame<'a> {
    pub fn initialize(chessboard_texture: &'a Texture,
                      pieces_texture: &'a Texture,
                      opengl_context: &'a OpenglContext,
                      chessboard_shader: &'a render_gl::Program,
                      possible_move_shader: &'a render_gl::Program) -> ChessGame<'a> {

        let resource_manager = ResourceManager::new(chessboard_texture, chessboard_shader, possible_move_shader, pieces_texture);
        let mut chessboard = Chessboard::new(&opengl_context, resource_manager);

        chessboard.init_pieces();
        ChessGame {
            chessboard
        }
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
        self.chessboard.render()
    }
}
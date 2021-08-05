use crate::render_gl;
use crate::texture::Texture;

pub struct ResourceManager<'a> {
    chessboard_texture: &'a Texture,
    chessboard_shader: &'a render_gl::Program,
    possible_move_shader: &'a render_gl::Program,
    pieces_sheet: &'a Texture,
}

impl<'a> ResourceManager<'a> {
    pub fn new(chessboard_texture: &'a Texture,
               chessboard_shader: &'a render_gl::Program,
               possible_move_shader: &'a render_gl::Program,
               pieces_sheet: &'a Texture) -> ResourceManager<'a> {
        ResourceManager {
            chessboard_shader,
            chessboard_texture,
            possible_move_shader,
            pieces_sheet,
        }
    }


    pub fn get_pieces_sheet(&self) -> &'a Texture {
        self.pieces_sheet
    }

    pub fn get_chessboard_texture(&self) -> &'a Texture {
        self.chessboard_texture
    }

    pub fn get_possible_move_shader(&self) -> &'a render_gl::Program {
        self.possible_move_shader
    }

    pub fn get_chessboard_shader(&self) -> &'a render_gl::Program {
        self.chessboard_shader
    }
}
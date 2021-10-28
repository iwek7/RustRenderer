use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::c_void;
use std::path::Path;
use std::rc::Rc;

use freetype;
use freetype::face::LoadFlag;
use freetype::ffi::FT_LOAD_RENDER;
use gl;
use soloud::Wav;

use crate::engine::api::audio::AudioResource;
use crate::engine::api::texture::{InternalFormat, Texture, TextureFilterType, TextureParams, TextureWrapType};
use crate::engine::render_gl;
use crate::engine::render_gl::{Shader, ShaderProgram};

pub trait ResourceManager {
    fn fetch_shader_program(&self, id: &str) -> Rc<ShaderProgram>;
    fn fetch_texture(&self, id: &str) -> Rc<Texture>;
    fn fetch_sprite_sheet(&self, id: &str, n_rows: u32, n_cols: u32) -> Rc<Texture>;
    fn fetch_audio(&self, id: &str) -> Rc<AudioResource>;
    fn fetch_font(&self, id: &str) -> Rc<SizedFont>;
}

pub struct SizedFont {
    characters: Vec<Character>,
}

impl SizedFont {
    pub fn new(face: &freetype::Face) -> SizedFont {
        face.set_char_size(0, 48 * 64, 96, 96).unwrap();
        let mut characters = vec!();
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        for i in 0..255 {
            face.load_char(i, LoadFlag::RENDER);
            characters.push(Character::new(face));
        }
        unsafe {
            gl::PixelStoref(gl::UNPACK_ALIGNMENT, 4 as gl::types::GLfloat);
        }
        SizedFont {
            characters
        }
    }

    pub fn get_char(&self, ch: char) -> &Character {
        &self.characters.get(ch as usize).unwrap()
    }
}

pub struct Character {
    texture: Rc<Texture>,
    size: glam::Vec2,
    bearing: glam::Vec2,
    advance: i64,
}

impl Character {
    pub fn new(configured_face: &freetype::Face) -> Character {
        let texture_params = TextureParams::new()
            .with_mag_filter(TextureFilterType::LINEAR)
            .with_min_filter(TextureFilterType::LINEAR)
            .with_x_wrap(TextureWrapType::CLAMP_TO_EDGE)
            .with_y_wrap(TextureWrapType::CLAMP_TO_EDGE);

        let glyph = configured_face.glyph();
        let bitmap = glyph.bitmap();

        let mut buffer = vec!();
        for y in 0..bitmap.rows() {
            for x in 0..bitmap.width() {
                // for whatever reason arrangement of bytes here is such that we need to go over rows from reverse order
                // to get mirror image of text
                // I suppose that I have mistake somewhere else in texture code but reverting characters here works for now :)
                buffer.push(bitmap.buffer()[(bitmap.width() * (bitmap.rows() - 1 - y) + x) as usize]);
            }
        }

        Character {
            texture: Rc::new(Texture::from_raw_data(buffer, bitmap.width(), bitmap.rows(), texture_params, 1, 1, InternalFormat::RED)),
            size: glam::vec2(bitmap.width() as f32, bitmap.rows() as f32),
            bearing: glam::vec2(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32),
            advance: glyph.advance().x,
        }
    }

    pub fn get_texture(&self) -> Rc<Texture> {
        Rc::clone(&self.texture)
    }

    pub fn get_size(&self) -> &glam::Vec2 {
        &self.size
    }

    pub fn get_bearing(&self) -> &glam::Vec2 {
        &self.bearing
    }

    pub fn get_advance(&self) -> &i64 {
        &self.advance
    }
}


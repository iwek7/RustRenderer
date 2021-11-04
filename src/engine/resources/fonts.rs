use std::rc::Rc;

use freetype::face::LoadFlag;

use crate::engine::api::texture::{InternalFormat, Sprite, Texture, TextureFilterType, TextureParams, TextureWrapType};

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
    sprite: Sprite,
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

        let tx = Texture::from_raw_data(buffer, bitmap.width(), bitmap.rows(), texture_params, InternalFormat::RED);
        let sprite = Sprite::new(Rc::new(tx));
        Character {
            sprite,
            size: glam::vec2(bitmap.width() as f32, bitmap.rows() as f32),
            bearing: glam::vec2(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32),
            advance: glyph.advance().x,
        }
    }

    pub fn get_sprite(&self) -> &Sprite {
        &self.sprite
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


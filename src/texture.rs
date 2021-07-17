use std::ffi::c_void;

use crate::resources::ImageData;

pub struct Texture {
    texture_id: gl::types::GLuint,
    pub topology: SpriteSheetTopology,
}

impl Texture {
    pub fn from_image(img_data: ImageData) -> Texture {
        return Texture::spritesheet_from_image(img_data, 1, 1);
    }

    pub fn spritesheet_from_image(img_data: ImageData, n_rows: u32, n_cols: u32) -> Texture {
        let mut texture_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }

        // todo: of course not always RGBA
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           img_data.width as i32,
                           img_data.height as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           img_data.image.into_raw().as_ptr() as *const _ as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        return Texture {
            texture_id,
            topology: SpriteSheetTopology { spritesheet_size: (img_data.width, img_data.height), n_rows, n_cols },
        };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    // todo drop
}

pub struct SpriteSheetTopology {
    spritesheet_size: (u32, u32),
    n_rows: u32,
    n_cols: u32
}

impl SpriteSheetTopology {
    pub fn get_sprite_coords(&self, row: u32, col: u32) -> Result<SpriteCoords, SpriteSheetError> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(SpriteSheetError::TopologyMismatch { message: format!("Max allowed dim is {} : {}", self.n_rows, self.n_cols) });
        }
        // todo: cache in member variable
        let single_sprite_size = (self.spritesheet_size.0 as f32 / self.n_cols as f32, self.spritesheet_size.1 as f32 / self.n_rows as f32);
        return Ok(SpriteCoords {
            top_right: ((single_sprite_size.0 * row as f32 + single_sprite_size.0) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * col as f32 + single_sprite_size.1) / self.spritesheet_size.1 as f32),
            bottom_right: ((single_sprite_size.0 * row as f32 + single_sprite_size.0) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * col as f32) / self.spritesheet_size.1 as f32),
            bottom_left: ((single_sprite_size.0 * row as f32) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * col as f32) / self.spritesheet_size.1 as f32),
            top_left: ((single_sprite_size.0 * row as f32) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * col as f32 + single_sprite_size.1) / self.spritesheet_size.1 as f32),
        });
    }
}

#[derive(Debug)]
pub enum SpriteSheetError {
    TopologyMismatch { message: String },
}

#[derive(Copy, Clone)]
pub struct SpriteCoords {
    pub top_right: (f32, f32),
    pub bottom_right: (f32, f32),
    pub bottom_left: (f32, f32),
    pub top_left: (f32, f32),
}


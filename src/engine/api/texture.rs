use std::ffi::c_void;

use crate::engine::resources::ImageData;

pub struct Texture {
    texture_id: gl::types::GLuint,
    // todo: make this private
    pub topology: SpriteSheetTopology,
    texture_params: TextureParams // todo: this should be part of sprite, not texture
}

impl Texture {
    pub fn from_image(img_data: ImageData, texture_params: TextureParams) -> Texture {
        return Texture::spritesheet_from_image(img_data, 1, 1, texture_params);
    }

    pub fn spritesheet_from_image(img_data: ImageData, n_rows: u32, n_cols: u32, texture_params: TextureParams) -> Texture {
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
            texture_params.set_params();
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        return Texture {
            texture_id,
            topology: SpriteSheetTopology { spritesheet_size: (img_data.width, img_data.height), n_rows, n_cols },
            texture_params
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

pub struct SpriteSheet {
    sprite_sheet: Texture,
}

pub struct SpriteSheetTopology {
    spritesheet_size: (u32, u32),
    n_rows: u32,
    n_cols: u32,
}

impl SpriteSheetTopology {
    pub fn get_sprite_coords(&self, row: u32, col: u32) -> Result<SpriteCoords, SpriteSheetError> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(SpriteSheetError::TopologyMismatch { message: format!("Max allowed dim is {} : {}", self.n_rows, self.n_cols) });
        }
        // todo: cache in member variable
        let single_sprite_size = (self.spritesheet_size.0 as f32 / self.n_cols as f32, self.spritesheet_size.1 as f32 / self.n_rows as f32);
        return Ok(SpriteCoords {
            top_right: ((single_sprite_size.0 * col as f32 + single_sprite_size.0) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * row as f32 + single_sprite_size.1) / self.spritesheet_size.1 as f32),
            bottom_right: ((single_sprite_size.0 * col as f32 + single_sprite_size.0) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * row as f32) / self.spritesheet_size.1 as f32),
            bottom_left: ((single_sprite_size.0 * col as f32) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * row as f32) / self.spritesheet_size.1 as f32),
            top_left: ((single_sprite_size.0 * col as f32) / self.spritesheet_size.0 as f32, (single_sprite_size.1 * row as f32 + single_sprite_size.1) / self.spritesheet_size.1 as f32),
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

#[derive(Clone)]
pub struct TextureParams {
    texture_min_filter: Option<TextureFilterType>,
    texture_mag_filter: Option<TextureFilterType>,
    texture_wrap_type_x: Option<TextureWrapType>,
    texture_wrap_type_y: Option<TextureWrapType>,
}

impl TextureParams {
    pub fn new() -> TextureParams {
        TextureParams {
            texture_min_filter: None,
            texture_mag_filter: None,
            texture_wrap_type_x: None,
            texture_wrap_type_y: None,
        }
    }

    pub fn with_min_filter(&self, filter_type: TextureFilterType) -> TextureParams {
        TextureParams {
            texture_min_filter: Some(filter_type),
            texture_mag_filter: self.texture_mag_filter.clone(),
            texture_wrap_type_x: self.texture_wrap_type_x.clone(),
            texture_wrap_type_y: self.texture_wrap_type_y.clone(),
        }
    }

    pub fn with_mag_filter(&self, filter_type: TextureFilterType) -> TextureParams {
        TextureParams {
            texture_min_filter: self.texture_min_filter.clone(),
            texture_mag_filter: Some(filter_type),
            texture_wrap_type_x: self.texture_wrap_type_x.clone(),
            texture_wrap_type_y: self.texture_wrap_type_y.clone(),
        }
    }

    pub fn with_x_wrap(&self, wrap_type: TextureWrapType) -> TextureParams {
        TextureParams {
            texture_min_filter: self.texture_min_filter.clone(),
            texture_mag_filter: self.texture_mag_filter.clone(),
            texture_wrap_type_x: Some(wrap_type),
            texture_wrap_type_y: self.texture_wrap_type_y.clone(),
        }
    }

    pub fn with_y_wrap(&self, wrap_type: TextureWrapType) -> TextureParams {
        TextureParams {
            texture_min_filter: self.texture_min_filter.clone(),
            texture_mag_filter: self.texture_mag_filter.clone(),
            texture_wrap_type_x: self.texture_wrap_type_x.clone(),
            texture_wrap_type_y: Some(wrap_type),
        }
    }

    // be weary to not make it public - else everybody will be able to set this anywhere which will lead to mess
    unsafe fn set_params(&self) {
        match &self.texture_min_filter {
            None => {}
            Some(min_filter) => {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter.to_gl_type() as i32);
            }
        }

        match &self.texture_mag_filter {
            None => {}
            Some(mag_filter) => {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter.to_gl_type() as i32);
            }
        }

        match &self.texture_wrap_type_x {
            None => {}
            Some(x_wrap) => {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, x_wrap.to_gl_type() as i32);
            }
        }

        match &self.texture_wrap_type_y {
            None => {}
            Some(y_wrap) => {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, y_wrap.to_gl_type() as i32);
            }
        }
    }
}

#[derive(Clone)]
pub enum TextureWrapType {
    REPEAT,
    MIRRORED_REPEAT,
    CLAMP_TO_EDGE,
    CLAMP_TO_BORDER,
}

impl TextureWrapType {
    fn to_gl_type(&self) -> gl::types::GLuint {
        match self {
            TextureWrapType::REPEAT => { gl::REPEAT }
            TextureWrapType::MIRRORED_REPEAT => { gl::MIRRORED_REPEAT }
            TextureWrapType::CLAMP_TO_EDGE => { gl::CLAMP_TO_EDGE }
            TextureWrapType::CLAMP_TO_BORDER => { gl::CLAMP_TO_BORDER }
        }
    }
}

#[derive(Clone)]
pub enum TextureFilterType {
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,
}

impl TextureFilterType {
    fn to_gl_type(&self) -> gl::types::GLuint {
        match self {
            TextureFilterType::NEAREST => { gl::NEAREST }
            TextureFilterType::LINEAR => { gl::LINEAR }
            TextureFilterType::NEAREST_MIPMAP_NEAREST => { gl::NEAREST_MIPMAP_NEAREST }
            TextureFilterType::LINEAR_MIPMAP_NEAREST => { gl::LINEAR_MIPMAP_NEAREST }
            TextureFilterType::NEAREST_MIPMAP_LINEAR => { gl::NEAREST_MIPMAP_LINEAR }
            TextureFilterType::LINEAR_MIPMAP_LINEAR => { gl::LINEAR_MIPMAP_LINEAR }
        }
    }
}

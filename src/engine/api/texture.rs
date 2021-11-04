use std::ffi::c_void;
use std::rc::Rc;

pub struct Texture {
    texture_id: gl::types::GLuint,
    texture_params: TextureParams,
    // todo: this should be part of sprite, not texture
    size: glam::Vec2,
}

impl Texture {
    pub fn from_image(data: Vec<u8>, width: i32, height: i32, texture_params: TextureParams) -> Texture {
        return Texture::from_raw_data(data, width, height, texture_params, InternalFormat::RGBA);
    }

    pub fn spritesheet_from_image(data: Vec<u8>, width: i32, height: i32, texture_params: TextureParams) -> Texture {
        return Texture::from_raw_data(data, width, height, texture_params, InternalFormat::RGBA);
    }

    pub fn from_raw_data(data: Vec<u8>, width: i32, height: i32, texture_params: TextureParams, internal_format: InternalFormat) -> Texture {
        let mut texture_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format.to_gl_type() as i32,
                width,
                height,
                0,
                internal_format.to_gl_type(),
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _ as *const c_void,
            );
            texture_params.set_params();
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        return Texture {
            texture_id,
            size: glam::vec2(height as f32, width as f32),
            texture_params,
        };
    }

    pub fn get_size(&self) -> &glam::Vec2 {
        &self.size
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

// sprite sheet should be separate class maybe?
#[derive(Clone)]
pub struct Sprite {
    texture: Rc<Texture>,
    topology: SpriteSheetTopology,
}

impl Sprite {
    pub fn new_spritesheet(texture: Rc<Texture>, n_rows: u32, n_cols: u32) -> Sprite {
        Sprite {
            texture: Rc::clone(&texture),
            topology: SpriteSheetTopology::new(texture.get_size().clone(), n_rows, n_cols),
        }
    }

    pub fn new(texture: Rc<Texture>) -> Sprite {
        Sprite {
            texture: Rc::clone(&texture),
            topology: SpriteSheetTopology::new(texture.get_size().clone(), 1, 1),
        }
    }

    pub fn start_drawing(&self) {
        self.texture.bind()
    }

    pub fn stop_drawing(&self) {
        self.texture.unbind()
    }

    pub fn get_texture_coords(&self) -> TextureCoords {
        self.topology.get_texture_coords(0, 0).unwrap()
    }

    pub fn get_texture_coords_from_spritesheet(&self, row: u32, col: u32) -> TextureCoords {
        self.topology.get_texture_coords(row, col).unwrap()
    }
}

#[derive(Clone)]
struct SpriteSheetTopology {
    spritesheet_size: glam::Vec2,
    n_rows: u32,
    n_cols: u32,
}

impl SpriteSheetTopology {
    fn new(spritesheet_size: glam::Vec2, n_rows: u32, n_cols: u32) -> SpriteSheetTopology {
        SpriteSheetTopology {
            spritesheet_size,
            n_rows,
            n_cols,
        }
    }

    fn get_texture_coords(&self, row: u32, col: u32) -> Result<TextureCoords, SpriteSheetError> {
        if row >= self.n_rows || col >= self.n_cols {
            return Err(SpriteSheetError::TopologyMismatch { message: format!("Max allowed dim is {} : {}", self.n_rows, self.n_cols) });
        }
        // todo: cache in member variable
        let single_sprite_size = (self.spritesheet_size.x as f32 / self.n_cols as f32, self.spritesheet_size.y as f32 / self.n_rows as f32);
        return Ok(TextureCoords {
            top_right: ((single_sprite_size.0 * col as f32 + single_sprite_size.0) / self.spritesheet_size.x as f32, (single_sprite_size.1 * row as f32 + single_sprite_size.1) / self.spritesheet_size.y as f32),
            bottom_right: ((single_sprite_size.0 * col as f32 + single_sprite_size.0) / self.spritesheet_size.x as f32, (single_sprite_size.1 * row as f32) / self.spritesheet_size.y as f32),
            bottom_left: ((single_sprite_size.0 * col as f32) / self.spritesheet_size.x as f32, (single_sprite_size.1 * row as f32) / self.spritesheet_size.y as f32),
            top_left: ((single_sprite_size.0 * col as f32) / self.spritesheet_size.x as f32, (single_sprite_size.1 * row as f32 + single_sprite_size.1) / self.spritesheet_size.y as f32),
        });
    }
}

#[derive(Debug)]
pub enum SpriteSheetError {
    TopologyMismatch { message: String },
}

#[derive(Copy, Clone)]
pub struct TextureCoords {
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

pub enum InternalFormat {
    RED,
    RGBA,
    RGB,
}

impl InternalFormat {
    pub fn to_gl_type(&self) -> u32 {
        match self {
            InternalFormat::RED => { gl::RED }
            InternalFormat::RGBA => { gl::RGBA }
            InternalFormat::RGB => { gl::RGB }
        }
    }
}
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
use crate::engine::resources::ResourceLoader;

// todo: this probably should not be here but be more generic class in engine
pub struct ResourceManager {
    textures_cache: ResourceCache<Texture>,
    shaders_cache: ResourceCache<ShaderProgram>,
    audio_cache: ResourceCache<AudioResource>,
    font_faces_cache: ResourceCache<Vec<u8>>,
    resource_loader: ResourceLoader,
    freetype_lib: freetype::Library,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            textures_cache: ResourceCache::new(),
            shaders_cache: ResourceCache::new(),
            audio_cache: ResourceCache::new(),
            font_faces_cache: ResourceCache::new(),
            resource_loader: ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap(), // todo: parametrize
            freetype_lib: freetype::Library::init().unwrap(),
        }
    }

    // todo: fetch shader and fetch texture contains the same logic
    pub fn fetch_shader_program(&self, id: &str) -> Rc<ShaderProgram> {
        self.shaders_cache.fetch(
            id,
            || ShaderProgram::from_res(&self.resource_loader, &id).unwrap(),
        )
    }

    pub fn fetch_texture(&self, id: &str) -> Rc<Texture> {
        self.textures_cache.fetch(id,
                                  || {
                                      let texture_data = self.resource_loader.load_image(&id);
                                      Texture::from_image(
                                          texture_data,
                                          TextureParams::new()
                                              .with_mag_filter(TextureFilterType::NEAREST)
                                              .with_min_filter(TextureFilterType::NEAREST),
                                      )
                                  },
        )
    }

    pub fn fetch_sprite_sheet(&self, id: &str, n_rows: u32, n_cols: u32) -> Rc<Texture> {
        self.textures_cache.fetch(id,
                                  || {
                                      let texture_data = self.resource_loader.load_image(&id);
                                      Texture::spritesheet_from_image(
                                          texture_data,
                                          n_rows,
                                          n_cols,
                                          TextureParams::new()
                                              .with_mag_filter(TextureFilterType::NEAREST)
                                              .with_min_filter(TextureFilterType::NEAREST),
                                      )
                                  },
        )
    }

    pub fn fetch_audio(&self, id: &str) -> Rc<AudioResource> {
        self.audio_cache.fetch(id, || self.resource_loader.load_audio(id))
    }

    pub fn fetch_font(&self, id: &str) -> Rc<SizedFont> {
        let raw_face = self.font_faces_cache.fetch(id, || self.resource_loader.load_font_face(id));
        let face = self.freetype_lib.new_memory_face(raw_face, 0).unwrap();
        Rc::new(SizedFont::new(&face))
    }
}

struct ResourceCache<T, > {
    data: RefCell<HashMap<String, Rc<T>>>,
}

impl<T> ResourceCache<T> {
    fn new() -> ResourceCache<T> {
        ResourceCache {
            data: RefCell::new(HashMap::new())
        }
    }

    // defining F as generic is required if F is to be closure
    fn fetch<F: Fn() -> T>(&self, id: &str, load_resource: F) -> Rc<T> {
        match self.data.borrow_mut().entry(id.to_string()) {
            Entry::Occupied(o) => { Rc::clone(&o.get()) }
            Entry::Vacant(v) => {
                let data = Rc::new(load_resource());
                Rc::clone(v.insert(data))
            }
        }
    }
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


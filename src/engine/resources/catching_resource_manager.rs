use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::engine::api::audio::AudioResource;
use crate::engine::api::resource_manager::{ResourceManager};
use crate::engine::api::texture::{Texture, TextureFilterType, TextureParams};
use crate::engine::render_gl::{ShaderProgram, ShaderType};
use crate::engine::resources::fonts::SizedFont;
use crate::engine::resources::resource_loader::ResourceLoader;

// todo: this probably should not be here but be more generic class in engine
pub struct CachingResourceManager {
    textures_cache: ResourceCache<Texture>,
    shaders_cache: ResourceCache<ShaderProgram>,
    audio_cache: ResourceCache<AudioResource>,
    font_faces_cache: ResourceCache<Vec<u8>>,
    resource_loader: ResourceLoader,
    freetype_lib: freetype::Library,
}

impl CachingResourceManager {
    pub fn new() -> CachingResourceManager {
        CachingResourceManager {
            textures_cache: ResourceCache::new(),
            shaders_cache: ResourceCache::new(),
            audio_cache: ResourceCache::new(),
            font_faces_cache: ResourceCache::new(),
            resource_loader: ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap(), // todo: parametrize
            freetype_lib: freetype::Library::init().unwrap(),
        }
    }
}

impl ResourceManager for CachingResourceManager {
    // todo: fetch shader and fetch texture contains the same logic
    fn fetch_shader_program(&self, id: &str) -> Rc<ShaderProgram> {
        self.shaders_cache.fetch(
            id,
            || {
                let v_shader = self.resource_loader.load_cstring(format!("{}{}", id, ShaderType::VERTEX.file_extension()).as_str()).unwrap();
                let f_shader = self.resource_loader.load_cstring(format!("{}{}", id, ShaderType::FRAG.file_extension()).as_str()).unwrap();


                ShaderProgram::new(&v_shader, &f_shader, &id).unwrap()
            },
        )
    }

    fn fetch_texture(&self, id: &str) -> Rc<Texture> {
        self.textures_cache.fetch(id,
                                  || {
                                      let texture_data = self.resource_loader.load_image(&id);
                                      Texture::from_image(
                                          texture_data.image.into_raw(),
                                          texture_data.width as i32,
                                          texture_data.height as i32,
                                          TextureParams::new()
                                              .with_mag_filter(TextureFilterType::NEAREST)
                                              .with_min_filter(TextureFilterType::NEAREST),
                                      )
                                  },
        )
    }

    fn fetch_sprite_sheet(&self, id: &str, n_rows: u32, n_cols: u32) -> Rc<Texture> {
        self.textures_cache.fetch(id,
                                  || {
                                      let texture_data = self.resource_loader.load_image(&id);
                                      Texture::spritesheet_from_image(
                                          texture_data.image.into_raw(),
                                          texture_data.width as i32,
                                          texture_data.height as i32,
                                          n_rows,
                                          n_cols,
                                          TextureParams::new()
                                              .with_mag_filter(TextureFilterType::NEAREST)
                                              .with_min_filter(TextureFilterType::NEAREST),
                                      )
                                  },
        )
    }

    fn fetch_audio(&self, id: &str) -> Rc<AudioResource> {
        self.audio_cache.fetch(id, || self.resource_loader.load_audio(id))
    }

    fn fetch_font(&self, id: &str) -> Rc<SizedFont> {
        let raw_face = self.font_faces_cache.fetch(id, || self.resource_loader.load_font_face(id));
        let face = self.freetype_lib.new_memory_face(raw_face, 0).unwrap();
        Rc::new(SizedFont::new(&face))
    }
}

struct ResourceCache<T> {
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
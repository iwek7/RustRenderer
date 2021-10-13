use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use soloud::Wav;
use crate::engine::api::audio::AudioResource;

use crate::engine::api::texture::{Texture, TextureFilterType, TextureParams};
use crate::engine::render_gl;
use crate::engine::render_gl::{Shader, ShaderProgram};
use crate::engine::resources::ResourceLoader;

// todo: this probably should not be here but be more generic class in engine
pub struct ResourceManager {
    textures_cache: ResourceCache<Texture>,
    shaders_cache: ResourceCache<ShaderProgram>,
    audio_cache: ResourceCache<AudioResource>,
    resource_loader: ResourceLoader,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            textures_cache: ResourceCache::new(),
            shaders_cache: ResourceCache::new(),
            audio_cache: ResourceCache::new(),
            resource_loader: ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap(), // todo: parametrize
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
        self.textures_cache.fetch( id,
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
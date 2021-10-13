use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use soloud::Wav;

use crate::api::texture::{Texture, TextureFilterType, TextureParams};
use crate::render_gl;
use crate::render_gl::{Shader, ShaderProgram};
use crate::resources::ResourceLoader;

// todo: this probably should not be here but be more generic class in engine
pub struct ResourceManager {
    textures: ResourceCache<Texture>,
    shaders: ResourceCache<ShaderProgram>,
    resource_loader: ResourceLoader,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            textures: ResourceCache::new(),
            shaders: ResourceCache::new(),
            resource_loader: ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap(), // todo: parametrize
        }
    }

    // todo: fetch shader and fetch texture contains the same logic
    pub fn fetch_shader_program(&self, id: &str) -> Rc<ShaderProgram> {
        self.shaders.fetch(
            id,
            || ShaderProgram::from_res(&self.resource_loader, &id).unwrap(),
        )
    }

    pub fn fetch_texture(&self, id: &str) -> Rc<Texture> {
        self.textures.fetch(
            id,
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
        self.textures.fetch(
            id,
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

    pub fn fetch_audio(&self, id: &str) -> Wav {
        self.resource_loader.load_audio(id)
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
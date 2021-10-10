use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use crate::render_gl;
use crate::render_gl::{Program, Shader};
use crate::resources::ResourceLoader;
use crate::texture::{Texture, TextureFilterType, TextureParams};

// todo: this probably should not be here but be more generic class in engine
pub struct ResourceManager {
    textures: RefCell<HashMap<String, Rc<Texture>>>,
    shaders: RefCell<HashMap<String, Rc<Program>>>,
    resource_loader: ResourceLoader,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            textures: RefCell::new(HashMap::new()),
            shaders: RefCell::new(HashMap::new()),
            resource_loader: ResourceLoader::from_relative_exe_path(Path::new("assets")).unwrap(), // todo: parametrize
        }
    }

    // todo: fetch shader and fetch texture contains the same logic
    pub fn fetch_shader_program(&self, id: &str) -> Rc<Program> {
        match self.shaders.borrow_mut().entry(id.to_string()) {
            Entry::Occupied(o) => { Rc::clone(&o.get()) }
            Entry::Vacant(v) => {
                let new_shader = Program::from_res(&self.resource_loader, &id).unwrap();
                Rc::clone(v.insert(Rc::new(new_shader)))
            }
        }
    }

    pub fn fetch_texture(&self, id: &str) -> Rc<Texture> {
        match self.textures.borrow_mut().entry(id.to_string()) {
            Entry::Occupied(o) => { Rc::clone(&o.get()) }
            Entry::Vacant(v) => {
                let texture_data = self.resource_loader.load_image(&id);
                let texture = Texture::from_image(
                    texture_data,
                    TextureParams::new()
                        .with_mag_filter(TextureFilterType::NEAREST)
                        .with_min_filter(TextureFilterType::NEAREST),
                );
                Rc::clone(v.insert(Rc::new(texture)))
            }
        }
    }

    pub fn fetch_sprite_sheet(&self, id: &str, n_rows: u32, n_cols: u32) -> Rc<Texture> {
        match self.textures.borrow_mut().entry(id.to_string()) {
            Entry::Occupied(o) => { Rc::clone(&o.get()) }
            Entry::Vacant(v) => {
                let texture_data = self.resource_loader.load_image(&id);
                let texture = Texture::spritesheet_from_image(
                    texture_data,
                    n_rows,
                    n_cols,
                    TextureParams::new()
                        .with_mag_filter(TextureFilterType::NEAREST)
                        .with_min_filter(TextureFilterType::NEAREST),
                );
                Rc::clone(v.insert(Rc::new(texture)))
            }
        }
    }
}
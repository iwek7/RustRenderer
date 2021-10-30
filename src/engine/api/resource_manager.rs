use std::rc::Rc;
use crate::engine::api::audio::AudioResource;
use crate::engine::api::texture::Texture;
use crate::engine::rendering::ShaderProgram;
use crate::engine::resources::fonts::SizedFont;

pub trait ResourceManager {
    fn fetch_shader_program(&self, id: &str) -> Rc<ShaderProgram>;
    fn fetch_texture(&self, id: &str) -> Rc<Texture>;
    fn fetch_sprite_sheet(&self, id: &str, n_rows: u32, n_cols: u32) -> Rc<Texture>;
    fn fetch_audio(&self, id: &str) -> Rc<AudioResource>;
    fn fetch_font(&self, id: &str) -> Rc<SizedFont>;
}

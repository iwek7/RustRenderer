use std::fs::File;
use std::io;
use std::rc::Rc;
use crate::engine::api::audio::AudioResource;
use crate::engine::api::texture::{Sprite};
use crate::engine::rendering::material::Material;
use crate::engine::rendering::ShaderProgram;
use crate::engine::resources::fonts::SizedFont;

pub trait ResourceManager {
    fn fetch_shader_program(&self, id: &str) -> Rc<ShaderProgram>;
    fn fetch_shader_material(&self, id: &str) -> Material;
    fn fetch_sprite(&self, id: &str) -> Sprite;
    fn fetch_sprite_sheet(&self, id: &str, n_rows: u32, n_cols: u32) -> Sprite;
    fn fetch_audio(&self, id: &str) -> Rc<AudioResource>;
    fn fetch_font(&self, id: &str) -> Rc<SizedFont>;
    fn read_file_lines(&self, id: &str) -> io::Lines<io::BufReader<File>>;
}

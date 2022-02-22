use sdl2::pixels::Color;
use crate::engine::rendering::material::Material;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseObject {
    id: i32,
}
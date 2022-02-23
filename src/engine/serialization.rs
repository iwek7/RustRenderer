use std::iter::Map;
use std::rc::Rc;

use sdl2::pixels::Color;
use serde::{Deserialize, Serialize};

use crate::engine::api::game_object::GameObject;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use crate::engine::rendering::material::Material;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDto {
    objects: Vec<GameObjectDto>,
}

impl ProjectDto {
    pub fn get_objects(&self) -> &Vec<GameObjectDto> {
        &self.objects
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GameObjectDto {
    id: i32,
    object_type: String,
    material: String,
    world_position: (f32, f32, f32),
    color: (f32, f32, f32, f32),
    properties: std::collections::HashMap<String, String>,
}


pub struct GameObjectMapper {
    engine_utilities: Rc<EngineUtilities>,
}


impl GameObjectMapper {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> GameObjectMapper {
        GameObjectMapper { engine_utilities }
    }

    pub fn map_dto_to_game_object(&self, dto: &GameObjectDto) -> Rectangle<ColoredVertexDataLayout> {
        let material = self.engine_utilities.get_resource_manager().fetch_shader_material(&dto.material);
        let size_x = dto.properties.get("sizeX").unwrap().parse::<f32>().unwrap();
        let size_y = dto.properties.get("sizeY").unwrap().parse::<f32>().unwrap();

        return
            Rectangle::new_colored(
                &dto.world_position.into(),
                &glam::vec2(size_x, size_y),
                material,
                dto.color.into(),
            );
    }
}

use std::rc::Rc;
use sdl2::pixels::Color;
use crate::engine::rendering::material::Material;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use serde::{Serialize, Deserialize};
use crate::engine::api::drawable::Drawable;
use crate::engine::api::engine_utilities::EngineUtilities;
use crate::engine::api::maths::rectangle::Rectangle;
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameObjectDto {
    id: i32,
    object_type: String,
    material: String,
}


pub struct GameObjectMapper {
    engine_utilities: Rc<EngineUtilities>,
}


impl GameObjectMapper {
    pub fn new(engine_utilities: Rc<EngineUtilities>) -> GameObjectMapper {
        GameObjectMapper { engine_utilities }
    }

    pub fn map_dto_to_game_object(&self, dto: GameObjectDto) -> Rectangle<ColoredVertexDataLayout> {
        let material = self.engine_utilities.get_resource_manager().fetch_shader_material(&dto.material);
        return
            Rectangle::new_colored(
                &glam::vec3(0.0, 0.0, 0.0),
                &glam::vec2(1.0, 1.0),
                material,
                (0.0, 0.741, 0.180, 1.0).into(),
            );
    }
}

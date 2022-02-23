use crate::engine::api::colour::WHITE;
use crate::engine::api::game_object::{BaseGameObject, GameObject};
use crate::engine::api::maths::segment::Segment;
use crate::engine::api::maths::vertex::ColoredVertexDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::rendering::material::Material;

pub struct CoordinateSystem {
    base_game_object: BaseGameObject,
    x_axis: Segment<ColoredVertexDataLayout>,
    y_axis: Segment<ColoredVertexDataLayout>,
    z_axis: Segment<ColoredVertexDataLayout>,
}

impl CoordinateSystem {
    pub fn new(material: Material) -> CoordinateSystem {
        let clr = WHITE;

        let mut x_axis = Segment::new(
            [
                ColoredVertexDataLayout { pos: (-100.0, 0.0, 0.0).into(), clr: clr.into() },
                ColoredVertexDataLayout { pos: (100.0, 0.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            material.clone(),
            glam::vec3(0.0, 0.0, 0.0),
        );
        let mut y_axis = Segment::new(
            [
                ColoredVertexDataLayout { pos: (0.0, -100.0, 0.0).into(), clr: clr.into() },
                ColoredVertexDataLayout { pos: (0.0, 100.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            material.clone(),
            glam::vec3(0.0, 0.0, 0.0),
        );
        let mut z_axis = Segment::new(
            [
                ColoredVertexDataLayout { pos: (0.0, 0.0, -100.0).into(), clr: clr.into() },
                ColoredVertexDataLayout { pos: (0.0, 0.0, 100.0).into(), clr: clr.into() },
            ],
            [0, 1],
            material,
            glam::vec3(0.0, 0.0, 0.0),
        );
        return CoordinateSystem {
            base_game_object: BaseGameObject::new(),
            x_axis,
            y_axis,
            z_axis,
        };
    }
}

impl GameObject for CoordinateSystem {
    fn render(&mut self, render_util: &RenderUtil) {
        self.x_axis.render(render_util);
        self.y_axis.render(render_util);
        self.z_axis.render(render_util);
    }

    fn base_game_object(&mut self) -> &mut BaseGameObject {
        &mut self.base_game_object
    }
}
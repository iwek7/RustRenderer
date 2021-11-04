use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::shapes_common::Area;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;

pub struct Segment<T> where T: VertexShaderDataLayout {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 2],
    indices: [i32; 2],
    material: Material,
    world_position: glam::Vec3,
}

/**
 usage example:
```
    let mut segment = Segment::new(
        [
            ColoredVertexData { pos: (0.0, 0.1, 0.0).into(), clr: (0.0, 0.0, 0.0).into() },
            ColoredVertexData { pos: (0.1, -0.1, 0.0).into(), clr: (0.0, 0.0, 0.0).into() },
        ],
        [0, 1],
        material
    );
```
 */
impl<'a, T: VertexShaderDataLayout> Segment<T> {
    pub fn new(vertices: [T; 2], indices: [i32; 2], material: Material, world_position: glam::Vec3) -> Segment<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            None,
        );

        Segment {
            drawing_component,
            vertices,
            indices,
            material,
            world_position,
        }
    }
}

impl<'a, T: VertexShaderDataLayout> Drawable for Segment<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            self.indices.len() as i32,
            gl::LINES,
            to_glam_vec(&self.get_pos()),
            render_util,
            &mut self.material,
        )
    }
}

impl<T: VertexShaderDataLayout> Area for Segment<T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        todo!()
    }

    fn area(&self) -> f32 {
        0.0
    }

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        self.world_position.into()
    }
}


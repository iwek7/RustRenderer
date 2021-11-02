use std::rc::Rc;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::vertex::VertexShaderDataLayout;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::glam_utils::to_glam_vec;
use crate::engine::rendering;
use crate::engine::rendering::material::Material;
use crate::engine::rendering::shape_drawing_component::ShapeDrawingComponent;

pub struct Segment<T> where T: VertexShaderDataLayout {
    drawing_component: ShapeDrawingComponent<T>,
    vertices: [T; 2],
    indices: [i32; 2],
    is_dragged: bool,
    material: Material
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
    pub fn new(vertices: [T; 2], indices: [i32; 2], material: Material) -> Segment<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            None,
        );

        Segment {
            drawing_component,
            vertices,
            indices,
            is_dragged: false,
            material,
        }
    }

    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose_deprecated(x, y, z);
        }
        self.drawing_component.bind_data(&self.vertices)
    }

    pub fn get_pos(&self) -> glam::Vec3 {
        to_glam_vec(&self.vertices[0].get_pos_deprecated())
    }
}

impl<'a, T: VertexShaderDataLayout> Drawable for Segment<T> {
    fn render(&mut self, render_util: &RenderUtil) {
        self.drawing_component.render(
            self.indices.len() as i32,
            gl::LINES,
            self.get_pos(),
            render_util,
            &mut self.material
        )
    }
}


use crate::glam_utils::to_glam_vec;
use crate::maths::vertex::VertexShaderDataConfigurer;
use crate::render_gl;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::renderer::RenderUtil;
use crate::api::drawable::Drawable;

pub struct Segment<'a, T> where T: VertexShaderDataConfigurer {
    drawing_component: ShapeDrawingComponent<'a, T>,
    vertices: [T; 2],
    indices: [i32; 2],
    is_dragged: bool,
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
          &shader_program
      );
  ```
 */
impl<'a, T: VertexShaderDataConfigurer> Segment<'a, T> {
    pub fn new(vertices: [T; 2], indices: [i32; 2], program: &render_gl::Program) -> Segment<T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            None,
            program,
        );

        Segment {
            drawing_component,
            vertices,
            indices,
            is_dragged: false,
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

impl<'a, T: VertexShaderDataConfigurer> Drawable for Segment<'a, T> {
    fn render(&self, render_util: &RenderUtil) {
        self.drawing_component.render(self.indices.len() as i32, gl::LINES, self.get_pos(), render_util)
    }
}


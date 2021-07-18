use crate::render_gl;
use crate::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::texture::Texture;
use crate::vertex::VertexShaderDataSetter;
use crate::chess::Draggable;

pub struct Triangle<'a, T: VertexShaderDataSetter> {
    open_gl_context: ShapeDrawingComponent<'a, T>,
    vertices: [T; 3],
    indices: [i32; 3],
    is_dragged: bool, // todo it should not be here ...
}

// todo: pass reference of texture here
impl<'a, T: VertexShaderDataSetter> Triangle<'a, T> {
    pub fn new(vertices: [T; 3], indices: [i32; 3], program: &'a render_gl::Program, texture: Option<&'a Texture>) -> Triangle<'a, T> {
        let open_gl_context = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            texture,
            program,
        );

        Triangle {
            open_gl_context,
            vertices,
            indices,
            is_dragged: false,
        }
    }

    // some algebra lib?
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose(x, y, z);
        }
        self.open_gl_context.bind_data(&self.vertices)
    }
}

impl<'a, T: VertexShaderDataSetter> Drawable for Triangle<'a, T> {
    fn render(&self) {
        self.open_gl_context.render(self.indices.len() as i32, gl::TRIANGLES)
    }
}

impl<'a, T: VertexShaderDataSetter + Clone> Area for Triangle<'a, T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32) {
                                                     let p = v.get_pos();
                                                      (p.0, p.1)
                                                  })
                                                  .collect(), );
    }

    fn area(&self) -> f32 {
        todo!()
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }

    fn get_pos(&self) -> (f32, f32, f32) {
        todo!()
    }
}

// todo: to separate file
pub trait Drawable {
    fn render(&self);
}



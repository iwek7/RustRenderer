use crate::{Draggable, render_gl};
use crate::maths::shapes_common::{Area, is_point_within_convex_polygon, ShapeDrawingComponent};
use crate::vertex::VertexShaderDataSetter;
use crate::texture::Texture;

pub struct Triangle<'a, T: VertexShaderDataSetter> {
    open_gl_context: ShapeDrawingComponent<'a, T>,
    vertices: [T; 3],
    indices: [i32; 3],
    is_dragged: bool, // todo it should not be here ...
}

// todo: pass reference of texture here
impl<'a, T: VertexShaderDataSetter> Triangle<'a, T> {
    pub fn new(vertices: [T; 3], indices: [i32; 3], program: &render_gl::Program, texture: Option<Texture>) -> Triangle<T> {
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
                                                  .map(|v| -> (f32, f32){ v.get_pos() })
                                                  .collect(), );
    }

    fn area(&self) -> f32 {
        todo!()
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }
}

impl<'a, T: VertexShaderDataSetter + Clone> Draggable for Triangle<'a, T> {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool {
        self.contains_point(mouse_pos)
    }

    fn handle_start_drag(&mut self) {
        // nothing
    }

    fn handle_drop(&mut self) {
        // nothing
    }

    fn handle_drag_pointer_move(&mut self, offset: &(f32, f32)) {
        self.move_by(offset.0, offset.1, 0.0)
    }
}


// todo: to separate file
pub trait Drawable {
    fn render(&self);
}



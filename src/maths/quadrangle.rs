use crate::{render_gl};
use crate::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::maths::triangle::Drawable;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::texture::Texture;
use crate::vertex::VertexShaderDataSetter;
use crate::mouse_drag_controller::Draggable;

// todo: reduce duplication https://users.rust-lang.org/t/how-to-implement-inheritance-like-feature-for-rust/31159
pub struct Quadrangle<'a, T> where T: VertexShaderDataSetter {
    drawing_component: ShapeDrawingComponent<'a, T>,
    vertices: [T; 4],
    indices: [i32; 6],
    is_dragged: bool, // todo: it should not be here
}

impl<'a, T: VertexShaderDataSetter> Quadrangle<'a, T> {
    pub fn new(vertices: [T; 4],
               indices: [i32; 6],
               program: &'a render_gl::Program,
               texture: Option<&'a Texture>) -> Quadrangle<'a, T> {
        let drawing_component = ShapeDrawingComponent::new(
            &vertices,
            &indices,
            texture,
            program,
        );
        Quadrangle {
            drawing_component,
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
        self.drawing_component.bind_data(&self.vertices)
    }
}

impl<'a, T: VertexShaderDataSetter> Drawable for Quadrangle<'a, T> {
    fn render(&self) {
        self.drawing_component.render(self.indices.len() as i32, gl::TRIANGLES)
    }
}

impl<'a, T: VertexShaderDataSetter> Area for Quadrangle<'a, T> {
    fn contains_point(&self, point: &(f32, f32)) -> bool {
        return is_point_within_convex_polygon(point,
                                              &self.vertices.iter()
                                                  .map(|v| -> (f32, f32){ v.get_pos() })
                                                  .collect(),
        );
    }

    fn area(&self) -> f32 {
        return 1.0;
    }

    fn num_vertices(&self) -> usize {
        return self.vertices.len();
    }
}

impl<'a, T: VertexShaderDataSetter> Draggable for Quadrangle<'a, T> {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool {
        self.contains_point(mouse_pos)
    }

    fn handle_start_drag(&mut self) {
        self.is_dragged = true
    }

    fn handle_drop(&mut self) {
        self.is_dragged = false
    }

    fn handle_drag_pointer_move(&mut self, offset: &(f32, f32)) {
        if self.is_dragged {
            self.move_by(offset.0, offset.1, 0.0)
        }
    }
}
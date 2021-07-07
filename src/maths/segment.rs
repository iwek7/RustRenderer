use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexShaderDataSetter;
use crate::render_gl;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;

pub struct Segment<'a, T> where T: VertexShaderDataSetter {
    drawing_component: ShapeDrawingComponent<'a, T>,
    vertices: [T; 2],
    indices: [i32; 2],
    is_dragged: bool,
}

impl<'a, T: VertexShaderDataSetter> Segment<'a, T> {
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
            vertex.transpose(x, y, z);
        }
        self.drawing_component.bind_data(&self.vertices)
    }
}

impl<'a, T: VertexShaderDataSetter> Drawable for Segment<'a, T> {
    fn render(&self) {
        self.drawing_component.render(self.indices.len() as i32, gl::LINES)
    }
}


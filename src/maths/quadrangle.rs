use crate::chess::Draggable;
use crate::maths::shapes_common::{Area, is_point_within_convex_polygon};
use crate::maths::triangle::Drawable;
use crate::render_gl;
use crate::render_gl::shape_drawing_component::ShapeDrawingComponent;
use crate::texture::Texture;
use crate::vertex::VertexShaderDataSetter;

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
    // opengl coords :(
    pub fn move_by(&mut self, x: f32, y: f32, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.transpose(x, y, z);
        }
        self.drawing_component.bind_data(&self.vertices)
    }

    // moves first vertex
    // does not support rotation
    pub fn move_to(&mut self, final_pos: &(f32, f32, f32)) {
        let current_pos = self.vertices[3].get_pos();
        self.move_by(
            final_pos.0 - current_pos.0,
            final_pos.1 - current_pos.1,
            final_pos.2 - current_pos.2,
        );
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
                                                  .map(|v| -> (f32, f32){
                                                      let p = v.get_pos();
                                                      (p.0, p.1)
                                                  })
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

// todo: move everything here away from here to pawn
impl<'a, T: VertexShaderDataSetter> Draggable for Quadrangle<'a, T> {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool {
        self.contains_point(mouse_pos)
    }

    fn handle_start_drag(&mut self) {
        self.is_dragged = true
    }

    fn handle_drop(&mut self, final_pos: Option<(f32, f32)>) {
        // todo: move this control to piece
        let unwr = final_pos.unwrap();
        println!("kappa {} {}", unwr.0, unwr.1);
        self.move_to(&(unwr.0, unwr.1, 0.0));
        self.is_dragged = false
    }

    fn handle_drag_pointer_move(&mut self, offset: &(f32, f32)) {
        if self.is_dragged {
            self.move_by(offset.0, offset.1, 0.0)
        }
    }

    fn is_dragged(&mut self) -> bool {
        self.is_dragged
    }
}
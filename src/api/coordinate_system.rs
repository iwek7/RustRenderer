use crate::maths::segment::Segment;
use crate::maths::vertex::ColoredVertexData;
use crate::render_gl::Program;
use crate::renderer::RenderUtil;
use crate::api::drawable::Drawable;

pub struct CoordinateSystem<'a> {
    x_axis: Segment<'a, ColoredVertexData>,
    y_axis: Segment<'a, ColoredVertexData>,
    z_axis: Segment<'a, ColoredVertexData>
}

impl<'a> CoordinateSystem<'a> {
    pub fn new(shader: &Program) -> CoordinateSystem {

        let clr =  (0.0, 0.0, 0.0, 1.0);

        let mut x_axis = Segment::new(
            [
                ColoredVertexData { pos: (-100.0, 0.0, 0.0).into(), clr: clr.into() },
                ColoredVertexData { pos: (100.0, 0.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            &shader,
        );
        let mut y_axis = Segment::new(
            [
                ColoredVertexData { pos: (0.0, -100.0, 0.0).into(), clr: clr.into() },
                ColoredVertexData { pos: (0.0, 100.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            &shader,
        );
        let mut z_axis = Segment::new(
            [
                ColoredVertexData { pos: (0.0, 0.0, -100.0).into(), clr: clr.into() },
                ColoredVertexData { pos: (0.0, 0.0, 100.0).into(), clr: clr.into() },
            ],
            [0, 1],
            &shader,
        );
        return CoordinateSystem {
            x_axis,
            y_axis,
            z_axis
        }
    }
}

impl<'a> Drawable for CoordinateSystem<'a> {
    fn render(&self, render_util: &RenderUtil) {
        self.x_axis.render(render_util);
        self.y_axis.render(render_util);
        self.z_axis.render(render_util);
    }
}
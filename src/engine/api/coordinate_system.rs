use std::rc::Rc;

use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::segment::Segment;
use crate::engine::api::maths::vertex::ColoredVertexData;
use crate::engine::api::render_util::RenderUtil;
use crate::engine::rendering::ShaderProgram;

pub struct CoordinateSystem {
    x_axis: Segment<ColoredVertexData>,
    y_axis: Segment<ColoredVertexData>,
    z_axis: Segment<ColoredVertexData>
}

impl CoordinateSystem {
    pub fn new(shader: Rc<ShaderProgram>) -> CoordinateSystem {

        let clr =  (0.0, 0.0, 0.0, 1.0);

        let mut x_axis = Segment::new(
            [
                ColoredVertexData { pos: (-100.0, 0.0, 0.0).into(), clr: clr.into() },
                ColoredVertexData { pos: (100.0, 0.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            Rc::clone(&shader),
        );
        let mut y_axis = Segment::new(
            [
                ColoredVertexData { pos: (0.0, -100.0, 0.0).into(), clr: clr.into() },
                ColoredVertexData { pos: (0.0, 100.0, 0.0).into(), clr: clr.into() },
            ],
            [0, 1],
            Rc::clone(&shader),
        );
        let mut z_axis = Segment::new(
            [
                ColoredVertexData { pos: (0.0, 0.0, -100.0).into(), clr: clr.into() },
                ColoredVertexData { pos: (0.0, 0.0, 100.0).into(), clr: clr.into() },
            ],
            [0, 1],
            Rc::clone(&shader),
        );
        return CoordinateSystem {
            x_axis,
            y_axis,
            z_axis
        }
    }
}

impl Drawable for CoordinateSystem {
    fn render(&self, render_util: &RenderUtil) {
        self.x_axis.render(render_util);
        self.y_axis.render(render_util);
        self.z_axis.render(render_util);
    }
}
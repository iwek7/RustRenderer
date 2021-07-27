use crate::{create_rect_coords_in_opengl_space, create_rect_coords_in_opengl_space_colored, render_gl};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexColored;
use crate::opengl_context::OpenglContext;

pub struct Field<'a> {
    // todo: those variables should not be mutable anyhow
    pub data: FieldData,
    pub x: i32,
    pub y: i32,
    possible_move_overlay: Quadrangle<'a, VertexColored>,
    pub is_possible_move: bool,
}

impl<'a> Field<'a> {
    pub fn new(col: u32, row: u32, x: i32, y: i32, field_size: i32, possible_move_shader: &'a render_gl::Program, opengl_context: &OpenglContext) -> Field<'a> {
        if col > 7 || row > 7 {
            panic!(format!("Trying to create field with row {} and col {}", row, col))
        }
        let col_name = String::from(match col {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            7 => "H",
            _ => panic!(format!("Trying to create field with row {} and col {}", row, col))
        });
        let name = format!("{}{}", col_name, (row + 1).to_string());
        let possible_move_overlay = Quadrangle::new(
            create_rect_coords_in_opengl_space_colored(&opengl_context, (x, y, 0), (field_size, field_size), (0.0, 1.0, 1.0, 0.5)),
            [0, 1, 3, 1, 2, 3],
            &possible_move_shader,
            None,
        );

        println!("{}", format!("creating field name {} row {} col {} x {} y {}", name, row, col, x, y));
        Field {
            data: FieldData {
                name,
                col,
                row,
            },
            x,
            y,
            possible_move_overlay,
            is_possible_move: false,
        }
    }

    pub fn get_position_3d(&self) -> (i32, i32, i32) {
        (self.x, self.y, 0)
    }
}


impl<'a> Drawable for Field<'a> {
    fn render(&self) {
        if self.is_possible_move {
            self.possible_move_overlay.render()
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldData {
    pub name: String,
    pub col: u32,
    pub row: u32,
}
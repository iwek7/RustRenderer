use crate::{create_rect_coords_in_opengl_space, create_rect_coords_in_opengl_space_colored, render_gl};
use crate::maths::quadrangle::Quadrangle;
use crate::maths::triangle::Drawable;
use crate::maths::vertex::VertexColored;
use crate::opengl_context::OpenglContext;

pub struct Field<'a> {
    // todo: those variables should not be mutable anyhow
    pub logic: FieldLogic,
    pub x: i32,
    pub y: i32,
    possible_move_overlay: Quadrangle<'a, VertexColored>,
    pub is_possible_move: bool,
}

impl<'a> Field<'a> {
    pub fn new(col: u32, row: u32, x: i32, y: i32, field_size: i32, possible_move_shader: &'a render_gl::Program, opengl_context: &OpenglContext) -> Field<'a> {
        let possible_move_overlay = Quadrangle::new(
            create_rect_coords_in_opengl_space_colored(&opengl_context, (x, y, 0), (field_size, field_size), (0.0, 0.741, 0.180, 1.0)),
            [0, 1, 3, 1, 2, 3],
            &possible_move_shader,
            None,
        );

        Field {
            logic: FieldLogic::from_coords(row, col),
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
pub struct FieldLogic {
    pub name: String,
    pub col: u32,
    pub row: u32,
}

impl FieldLogic {
    pub fn from_coords(row: u32, col: u32) -> FieldLogic {
        if !FieldLogic::is_legal_field_coord(&(row as i32)) {
            panic!(format!("Trying to create field with invalid row {}", row))
        }

        if !FieldLogic::is_legal_field_coord(&(col as i32)) {
            panic!(format!("Trying to create field with invalid col {}", col))
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
        FieldLogic {
            name,
            row,
            col,
        }
    }

    pub fn from_string(str: &str) -> FieldLogic {
        let name = String::from(str);
        if name.len() != 2 {
            panic!(format!("Trying to parse field with invalid name {}", name))
        }
        let col = match name.chars().nth(0).unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => panic!(format!("Trying to create invalid field {}", name))
        };
        let row = name.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;
        if !(0..8).contains(&row) {
            panic!(format!("Trying to create invalid field {}", name))
        }

        FieldLogic {
            name: String::from(str),
            col,
            row,
        }
    }

    pub fn get_offset_field(&self, col_offset: i32, row_offset: i32) -> Option<FieldLogic> {
        let new_row = self.row as i32 + row_offset;
        if FieldLogic::is_legal_field_coord(&new_row) {
            return None
        }

        let new_col = self.col as i32 + col_offset;
        if FieldLogic::is_legal_field_coord(&new_col) {
            return None
        }

        Some(FieldLogic::from_coords((new_row as u32), (new_col as u32)))
    }

    fn is_legal_field_coord(coord: &i32) -> bool {
        (0_i32..8_i32).contains(&coord)
    }
}
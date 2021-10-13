use std::rc::Rc;

use crate::chess::allowed_move::ActionType;
use crate::create_rect_coords_colored_deprecated;
use crate::engine::api::drawable::Drawable;
use crate::engine::api::maths::quadrangle::Quadrangle;
use crate::engine::api::maths::vertex::ColoredVertexData;
use crate::engine::render_gl;
use crate::renderer::RenderUtil;

pub struct Field {
    // todo: those variables should not be mutable anyhow
    pub logic: FieldLogic,
    pub x: f32,
    pub y: f32,
    possible_move_overlay: Quadrangle<ColoredVertexData>,
    possible_capture_overlay: Quadrangle<ColoredVertexData>,
    current_field_overlay: Quadrangle<ColoredVertexData>,
    is_possible_move: bool,
    is_possible_capture: bool,
    pub is_current_field: bool,
}

impl Field {
    pub fn new(col: u32, row: u32, x: f32, y: f32, field_size: f32, possible_move_shader: Rc<render_gl::ShaderProgram>) -> Field {
        let possible_move_overlay = Quadrangle::new(
            create_rect_coords_colored_deprecated((x, y, 0.0), (field_size, field_size), (0.0, 0.741, 0.180, 1.0)),
            [0, 1, 3, 1, 2, 3],
            Rc::clone(&possible_move_shader),
            None,
        );

        let possible_capture_overlay = Quadrangle::new(
            create_rect_coords_colored_deprecated((x, y, 0.0), (field_size, field_size), (0.992, 0.070, 0.070, 0.5)),
            [0, 1, 3, 1, 2, 3],
            Rc::clone(&possible_move_shader),
            None,
        );

        let current_field_overlay = Quadrangle::new(
            create_rect_coords_colored_deprecated((x, y, 0.0), (field_size, field_size), (0.937, 0.941, 0.458, 0.5)),
            [0, 1, 3, 1, 2, 3],
            Rc::clone(&possible_move_shader),
            None,
        );

        Field {
            logic: FieldLogic::from_coords(row, col),
            x,
            y,
            possible_move_overlay,
            possible_capture_overlay,
            current_field_overlay,
            is_possible_move: false,
            is_possible_capture: false,
            is_current_field: false,
        }
    }

    pub fn get_position_3d(&self) -> (f32, f32, f32) {
        (self.x, self.y, 0.0)
    }

    pub fn update_with_allowed_move(&mut self, move_type: &ActionType) {
        match move_type {
            ActionType::MOVE | ActionType::PROMOTION => { self.is_possible_move = true }
            ActionType::EN_PASSABLE_MOVE { en_passant_target_field } => { self.is_possible_move = true }
            ActionType::COMPOSITE_MOVE { accompanying_move } => { self.is_possible_move = true }
            ActionType::CAPTURE { captured_piece } | ActionType::CAPTURE_PROMOTION { captured_piece } => { self.is_possible_capture = true }
            _ => {}
        }
    }

    pub fn clear_possible_moves_overlay(&mut self) {
        self.is_possible_capture = false;
        self.is_possible_move = false;
        self.is_current_field = false;
    }
}

impl Drawable for Field {
    fn render(&self, render_util: &RenderUtil) {
        if self.is_possible_move {
            self.possible_move_overlay.render(render_util)
        } else if self.is_possible_capture {
            self.possible_capture_overlay.render(render_util)
        } else if self.is_current_field {
            self.current_field_overlay.render(render_util)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldLogic {
    pub name: String,
    pub col: u32,
    pub row: u32,
}

impl FieldLogic {
    pub fn from_coords(row: u32, col: u32) -> FieldLogic {
        if !FieldLogic::is_legal_field_coord(&(row as i32)) {
            panic!("Trying to create field with invalid row {}", row)
        }

        if !FieldLogic::is_legal_field_coord(&(col as i32)) {
            panic!("Trying to create field with invalid col {}", col)
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
            _ => panic!("Trying to create field with row {} and col {}", row, col)
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
            panic!("Trying to parse field with invalid name {}", name)
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
            _ => panic!("Trying to create invalid field {}", name)
        };
        let row = name.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;
        if !(0..8).contains(&row) {
            panic!("Trying to create invalid field {}", name)
        }

        FieldLogic {
            name: String::from(str),
            col,
            row,
        }
    }

    pub fn get_offset_field(&self, col_offset: i32, row_offset: i32) -> Option<FieldLogic> {
        let new_row = self.row as i32 + row_offset;
        if !FieldLogic::is_legal_field_coord(&new_row) {
            return None;
        }

        let new_col = self.col as i32 + col_offset;
        if !FieldLogic::is_legal_field_coord(&new_col) {
            return None;
        }

        Some(FieldLogic::from_coords(new_row as u32, new_col as u32))
    }

    fn is_legal_field_coord(coord: &i32) -> bool {
        (0_i32..8_i32).contains(&coord)
    }
}
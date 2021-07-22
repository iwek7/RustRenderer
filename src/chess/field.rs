#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub col: u32,
    pub row: u32,
    pub x: i32,
    pub y: i32,
}

impl Field {
    pub fn of_position(col: u32, row: u32, x: i32, y: i32) -> Field {
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
        println!("{}", format!("creating field name {} row {} col {} x {} y {}", name, row, col, x, y));
        Field {
            name,
            col,
            row,
            x,
            y,
        }
    }

    pub fn get_position_3d(&self) -> (i32, i32, i32) {
        (self.x, self.y, 0)
    }
}
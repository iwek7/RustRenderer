#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Side {
    BLACK,
    WHITE,
}

impl Side {
    pub fn adjust_pawn_move_offset(&self, row_offset: &i32) -> i32 {
        match self {
            Side::BLACK => { -row_offset.clone() }
            Side::WHITE => { row_offset.clone() }
        }
    }

    pub fn get_other(&self) -> Side {
        match self {
            Side::BLACK => { Side::WHITE }
            Side::WHITE => { Side::BLACK }
        }
    }

    pub fn get_last_rank_row(&self) -> u32 {
        match self {
            Side::BLACK => { 0 }
            Side::WHITE => { 7 }
        }
    }
}


pub trait VectorExtension<T> {
    fn push_if_exists(&mut self, opt: Option<T>);
}

impl<T> VectorExtension<T> for Vec<T> {
    fn push_if_exists(&mut self, opt: Option<T>) {
        match opt {
            None => {}
            Some(val) => { self.push(val) }
        }
    }
}
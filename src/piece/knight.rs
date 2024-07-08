use crate::{
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Knight {
    pub player: Player,
}

impl ChessPieceTrait for Knight {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        Option::None // temporary value
    }
}

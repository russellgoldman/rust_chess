use crate::{
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Rook {
    pub player: Player,
}

impl ChessPieceTrait for Rook {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        Option::None // temporary value
    }
}

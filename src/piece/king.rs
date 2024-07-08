use crate::{
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct King {
    pub player: Player,
}

impl ChessPieceTrait for King {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        Option::None // temporary value
    }
}

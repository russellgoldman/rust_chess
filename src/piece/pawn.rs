use crate::{
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Pawn {
    pub player: Player,
    pub has_been_moved: bool,
}

impl ChessPieceTrait for Pawn {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        Option::None // temporary value
    }
}

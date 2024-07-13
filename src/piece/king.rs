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
        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(K)"),
            Player::White => print!("[K]"),
        }
    }
}

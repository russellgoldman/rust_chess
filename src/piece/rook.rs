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
        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(R)"),
            Player::White => print!("[R]"),
        }
    }
}

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
        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(k)"),
            Player::White => print!("[k]"),
        }
    }
}

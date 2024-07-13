use crate::{
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Queen {
    pub player: Player,
}

impl ChessPieceTrait for Queen {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(Q)"),
            Player::White => print!("[Q]"),
        }
    }
}

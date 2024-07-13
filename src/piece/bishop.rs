use crate::{
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Bishop {
    pub player: Player,
}

impl ChessPieceTrait for Bishop {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(B)"),
            Player::White => print!("[B]"),
        }
    }
}

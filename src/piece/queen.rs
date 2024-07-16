use crate::{
    game::ChessBoard,
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Queen {
    pub player: Player,
    pub position: BoardPosition,
}

impl ChessPieceTrait for Queen {
    fn valid_moves(&self, board: &ChessBoard) -> Option<Vec<BoardPosition>> {
        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(Q)"),
            Player::White => print!("[Q]"),
        }
    }
}

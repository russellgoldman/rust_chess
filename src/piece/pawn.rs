use crate::{
    game::ChessBoard,
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Pawn {
    pub player: Player,
    pub position: BoardPosition,
    pub has_been_moved: bool,
}

impl ChessPieceTrait for Pawn {
    fn valid_moves(&self, board: &ChessBoard) -> Option<Vec<BoardPosition>> {
        /*
            Look at the current position of the piece

            - Pawn's can move one or two spaces forward only if they haven't been moved in the game yet
            - Otherwise, Pawns' can only move one space forward
            - Pawn's can capture enemy pieces by moving one space forward diagonally
            - White Pawn's can only move forward in the range of rows 1 - 7 (increasing)
            - Black Pawn's can only move forward in the range of rows 6 - 0 (decreasing)

            For simplicity, we will not be implementing the En Passant rule.
        */

        None // temporary value
    }

    fn print_piece(&self) {
        match self.player {
            Player::Black => print!("(P)"),
            Player::White => print!("[P]"),
        }
    }
}

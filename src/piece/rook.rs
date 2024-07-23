use crate::{
    game::board::ChessBoard,
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Rook {
    pub player: Player,
    pub position: BoardPosition,
}

impl ChessPieceTrait for Rook {
    fn get_player(&self) -> &Player {
        &self.player
    }

    fn valid_moves(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        todo!();
    }

    fn valid_captures(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        todo!()
    }

    fn display_piece_on_board(&self) {
        match self.player {
            Player::Black => print!("(R)"),
            Player::White => print!("[R]"),
        }
    }

    fn get_piece_name(&self) -> &str {
        "Rook"
    }

    fn get_board_position(&self) -> &BoardPosition {
        &self.position
    }
}

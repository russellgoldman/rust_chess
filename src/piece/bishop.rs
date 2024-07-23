use crate::{
    game::board::ChessBoard,
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

#[derive(Debug)]
pub struct Bishop {
    pub player: Player,
    pub position: BoardPosition,
}

impl ChessPieceTrait for Bishop {
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
            Player::Black => print!("(B)"),
            Player::White => print!("[B]"),
        }
    }

    fn get_piece_name(&self) -> &str {
        "Bishop"
    }

    fn get_board_position(&self) -> &BoardPosition {
        &self.position
    }
}

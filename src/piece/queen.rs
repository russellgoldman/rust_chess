use crate::{
    game::board::ChessBoard,
    piece::{BoardPosition, ChessPieceTrait, PieceMoveData},
    player::Player,
};

#[derive(Debug)]
pub struct Queen {
    pub player: Player,
    pub position: BoardPosition,
}

impl ChessPieceTrait for Queen {
    fn get_player(&self) -> &Player {
        &self.player
    }

    fn valid_moves_and_captures(&self, board: &ChessBoard) -> PieceMoveData {
        todo!();
    }

    fn display_piece_on_board(&self) {
        match self.player {
            Player::Black => print!("(Q)"),
            Player::White => print!("[Q]"),
        }
    }

    fn get_piece_name(&self) -> &str {
        "Queen"
    }

    fn get_board_position(&self) -> &BoardPosition {
        &self.position
    }
}

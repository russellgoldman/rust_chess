use crate::piece::ChessPiece;

use super::BoardPosition;
use std::collections::HashMap;

pub struct ChessBoardData {
    pub board: ChessBoard,
    // black_pieces and white_pieces need to be updated when any pieces are updated on the board
    pub black_pieces: HashMap<BoardPosition, ChessPiece>,
    pub white_pieces: HashMap<BoardPosition, ChessPiece>,
}
pub type ChessBoard = [ChessRow; 8];
pub type ChessRow = [Option<ChessPiece>; 8];

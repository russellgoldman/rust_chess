use crate::piece::ChessPiece;

pub type ChessBoard = Vec<Vec<Option<ChessPiece>>>;

#[derive(Debug)]
pub struct BoardPosition {
    pub row_index: i32,
    pub column_index: i32,
}

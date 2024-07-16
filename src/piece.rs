pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::game::{BoardPosition, ChessBoard};
use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

pub trait ChessPieceTrait {
    fn valid_moves(&self, board: &ChessBoard) -> Option<Vec<BoardPosition>>;
    fn print_piece(&self);
}

#[derive(Debug)]
pub enum ChessPiece {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl ChessPieceTrait for ChessPiece {
    fn valid_moves(&self, board: &ChessBoard) -> Option<Vec<BoardPosition>> {
        match self {
            ChessPiece::Pawn(pawn) => pawn.valid_moves(board),
            ChessPiece::Bishop(bishop) => bishop.valid_moves(board),
            ChessPiece::Knight(knight) => knight.valid_moves(board),
            ChessPiece::Rook(rook) => rook.valid_moves(board),
            ChessPiece::Queen(queen) => queen.valid_moves(board),
            ChessPiece::King(king) => king.valid_moves(board),
        }
    }

    fn print_piece(&self) {
        match self {
            ChessPiece::Pawn(pawn) => pawn.print_piece(),
            ChessPiece::Bishop(bishop) => bishop.print_piece(),
            ChessPiece::Knight(knight) => knight.print_piece(),
            ChessPiece::Rook(rook) => rook.print_piece(),
            ChessPiece::Queen(queen) => queen.print_piece(),
            ChessPiece::King(king) => king.print_piece(),
        }
    }
}

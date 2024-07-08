pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::board::BoardPosition;
use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;

pub trait ChessPieceTrait {
    fn valid_moves(&self) -> Option<Vec<BoardPosition>>;
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
    fn valid_moves(&self) -> Option<Vec<BoardPosition>> {
        match self {
            ChessPiece::Pawn(pawn) => pawn.valid_moves(),
            ChessPiece::Bishop(bishop) => bishop.valid_moves(),
            ChessPiece::Knight(knight) => knight.valid_moves(),
            ChessPiece::Rook(rook) => rook.valid_moves(),
            ChessPiece::Queen(queen) => queen.valid_moves(),
            ChessPiece::King(king) => king.valid_moves(),
        }
    }
}

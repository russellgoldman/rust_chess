pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::game::board::ChessBoard;
use crate::game::BoardPosition;
use crate::player::Player;
use bishop::Bishop;
use king::King;
use knight::Knight;
use pawn::Pawn;
use queen::Queen;
use rook::Rook;
use std::fmt;

pub trait ChessPieceTrait {
    fn get_player(&self) -> &Player;
    fn valid_moves(&self, board: &ChessBoard) -> Vec<BoardPosition>;
    fn valid_captures(&self, board: &ChessBoard) -> Vec<BoardPosition>;
    fn display_piece_on_board(&self);
    fn get_piece_name(&self) -> &str;
    fn get_board_position(&self) -> &BoardPosition;
    // Default trait method
    fn get_position_str(&self) -> String {
        format!(
            "{} ({}{})",
            self.get_piece_name(),
            self.get_board_position().get_column_letter(),
            self.get_board_position().row_index + 1
        )
    }
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

impl fmt::Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_position_str())
    }
}

impl ChessPieceTrait for ChessPiece {
    fn get_player(&self) -> &Player {
        match self {
            ChessPiece::Pawn(pawn) => pawn.get_player(),
            ChessPiece::Bishop(bishop) => bishop.get_player(),
            ChessPiece::Knight(knight) => knight.get_player(),
            ChessPiece::Rook(rook) => rook.get_player(),
            ChessPiece::Queen(queen) => queen.get_player(),
            ChessPiece::King(king) => king.get_player(),
        }
    }

    fn valid_moves(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        match self {
            ChessPiece::Pawn(pawn) => pawn.valid_moves(board),
            ChessPiece::Bishop(bishop) => bishop.valid_moves(board),
            ChessPiece::Knight(knight) => knight.valid_moves(board),
            ChessPiece::Rook(rook) => rook.valid_moves(board),
            ChessPiece::Queen(queen) => queen.valid_moves(board),
            ChessPiece::King(king) => king.valid_moves(board),
        }
    }

    fn valid_captures(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        match self {
            ChessPiece::Pawn(pawn) => pawn.valid_captures(board),
            ChessPiece::Bishop(bishop) => bishop.valid_captures(board),
            ChessPiece::Knight(knight) => knight.valid_captures(board),
            ChessPiece::Rook(rook) => rook.valid_captures(board),
            ChessPiece::Queen(queen) => queen.valid_captures(board),
            ChessPiece::King(king) => king.valid_captures(board),
        }
    }

    fn display_piece_on_board(&self) {
        match self {
            ChessPiece::Pawn(pawn) => pawn.display_piece_on_board(),
            ChessPiece::Bishop(bishop) => bishop.display_piece_on_board(),
            ChessPiece::Knight(knight) => knight.display_piece_on_board(),
            ChessPiece::Rook(rook) => rook.display_piece_on_board(),
            ChessPiece::Queen(queen) => queen.display_piece_on_board(),
            ChessPiece::King(king) => king.display_piece_on_board(),
        }
    }

    fn get_piece_name(&self) -> &str {
        match self {
            ChessPiece::Pawn(pawn) => pawn.get_piece_name(),
            ChessPiece::Bishop(bishop) => bishop.get_piece_name(),
            ChessPiece::Knight(knight) => knight.get_piece_name(),
            ChessPiece::Rook(rook) => rook.get_piece_name(),
            ChessPiece::Queen(queen) => queen.get_piece_name(),
            ChessPiece::King(king) => king.get_piece_name(),
        }
    }

    fn get_board_position(&self) -> &BoardPosition {
        match self {
            ChessPiece::Pawn(pawn) => pawn.get_board_position(),
            ChessPiece::Bishop(bishop) => bishop.get_board_position(),
            ChessPiece::Knight(knight) => knight.get_board_position(),
            ChessPiece::Rook(rook) => rook.get_board_position(),
            ChessPiece::Queen(queen) => queen.get_board_position(),
            ChessPiece::King(king) => king.get_board_position(),
        }
    }
}

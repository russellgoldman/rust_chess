pub mod board;

use crate::{
    piece::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        ChessPiece, ChessPieceTrait,
    },
    player::Player,
};
use board::{ChessBoard, ChessBoardData};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct BoardPosition {
    pub row_index: usize,
    pub column_index: usize,
}
pub struct CandidateBoardPosition {
    pub row_index: i32,
    pub column_index: i32,
}
impl CandidateBoardPosition {
    pub fn validate_move(&self, board: &ChessBoard) -> Option<BoardPosition> {
        let is_valid_index: bool = {
            let board_size = board.len() as i32; // Assuming a square board
            self.row_index >= 0
                && self.row_index < board_size
                && self.column_index >= 0
                && self.column_index < board_size
        };

        let valid_move: Option<BoardPosition> = if is_valid_index {
            Some(BoardPosition {
                row_index: self.row_index as usize,
                column_index: self.column_index as usize,
            })
        } else {
            None
        };

        valid_move
    }

    pub fn validate_capture(&self, player: &Player, board: &ChessBoard) -> Option<BoardPosition> {
        /*
            If the value prepending the ? operator is Some(), the value will be returned into the
            valid_move variable. Otherwise, the function will return None immediately.
        */
        let valid_move: BoardPosition = self.validate_move(board)?;
        let new_position: Option<&ChessPiece> =
            board[valid_move.row_index][valid_move.column_index].as_ref();

        let valid_capture = match new_position {
            Some(piece) if piece.get_player() != player => Some(valid_move),
            _ => None, // catch-all if new_position is None, or new_position is Some but piece.player() == player
        };

        valid_capture
    }
}
impl BoardPosition {
    pub fn get_column_letter(&self) -> char {
        match self.column_index {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Invalid column_index: {}", self.column_index),
        }
    }
}
pub struct ChessGame {
    pub board_data: ChessBoardData,
    pub turn: u32, // since white starts first, if turn % 2 == 0 means white's turn, otherwise black's turn
}

impl ChessGame {
    pub fn new() -> Self {
        fn get_player_by_initial_row(row_index: usize) -> Player {
            match row_index {
                // White Player start occupied space (row indices 0, 1)
                0 | 1 => Player::White,
                // Black Player start occupied space (row indices 6, 7)
                6 | 7 => Player::Black,
                _ => panic!("Given row_index: {row_index}. This should not be evaluated as a player starting row index.")
            }
        }
        fn get_piece_by_initial_position(row_index: usize, column_index: usize) -> ChessPiece {
            let player = get_player_by_initial_row(row_index);
            match row_index {
                0 | 7 => match column_index {
                    0 | 7 => ChessPiece::Rook(Rook { player, position: BoardPosition { row_index, column_index } }),
                    1 | 6 => ChessPiece::Knight(Knight { player, position: BoardPosition { row_index, column_index } }),
                    2 | 5 => ChessPiece::Bishop(Bishop { player, position: BoardPosition { row_index, column_index } }),
                    3 => ChessPiece::Queen(Queen { player, position: BoardPosition { row_index, column_index } }),
                    4 => ChessPiece::King(King { player, position: BoardPosition { row_index, column_index } }),
                    _ => panic!("Invalid {column_index}"),
                },
                1 | 6 => ChessPiece::Pawn(Pawn { player, position: BoardPosition { row_index, column_index }, has_been_moved: false }),
                _ => panic!("Given row_index: {row_index}. This should not be evaluated as a starting row index.")
            }
        }

        // Chess board should be an 8x8 2d vector
        let mut board: ChessBoard = std::array::from_fn(|_| std::array::from_fn(|_| None));
        let mut black_pieces: HashMap<BoardPosition, ChessPiece> = HashMap::new();
        let mut white_pieces: HashMap<BoardPosition, ChessPiece> = HashMap::new();

        for row_index in 0..=7 {
            for column_index in 0..=7 {
                if row_index >= 2 && row_index <= 5 {
                    // Non-occupied space (row indices 2, 3, 4, 5)
                    // Since we initialize pieces to None by default, we can continue
                    continue;
                } else {
                    let piece: ChessPiece = get_piece_by_initial_position(row_index, column_index);
                    board[row_index][column_index] = Some(piece);

                    let player = get_player_by_initial_row(row_index);
                    let piece: ChessPiece = get_piece_by_initial_position(row_index, column_index);
                    match player {
                        Player::Black => black_pieces.insert(
                            BoardPosition {
                                row_index: row_index as usize,
                                column_index: column_index as usize,
                            },
                            piece,
                        ),
                        Player::White => white_pieces.insert(
                            BoardPosition {
                                row_index: row_index as usize,
                                column_index: column_index as usize,
                            },
                            piece,
                        ),
                    };
                }
            }
        }

        Self {
            board_data: ChessBoardData {
                board,
                black_pieces,
                white_pieces,
            },
            turn: 0,
        }
    }

    pub fn display_board(&self) {
        println!();
        println!("    {}[Black]{}", " ".repeat(20), " ".repeat(20));
        println!();
        println!(
            "{}a{}b{}c{}d{}e{}f{}g{}h",
            " ".repeat(7),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
        );
        println!("    {}", "-".repeat(49));
        for (row_index, row) in self.board_data.board.iter().enumerate() {
            print!(" {}  |", self.board_data.board.len() - row_index);
            for (column_index, piece) in row.iter().enumerate() {
                print!(" ");
                match piece {
                    Some(piece) => piece.display_piece_on_board(),
                    None => print!("   "),
                }
                if column_index < row.len() - 1 {
                    print!(" |");
                }
            }
            print!(" |  {}", self.board_data.board.len() - row_index);
            println!();
            println!("    {}", "-".repeat(49));
        }
        println!(
            "{}a{}b{}c{}d{}e{}f{}g{}h",
            " ".repeat(7),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
            " ".repeat(5),
        );
        println!();
        println!("    {}(White){}", " ".repeat(20), " ".repeat(20));
        println!();
    }
}

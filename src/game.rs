pub mod board;
pub mod board_position;

use crate::{
    piece::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        ChessPiece, ChessPieceTrait,
    },
    player::Player,
};
use board::{initialize_empty_board, ChessBoard, ChessBoardData};
use board_position::BoardPosition;
use std::collections::HashMap;

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
        let mut board: ChessBoard = initialize_empty_board();
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

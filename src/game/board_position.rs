use crate::{
    piece::{ChessPiece, ChessPieceTrait},
    player::Player,
};

use super::board::ChessBoard;
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
    fn is_position_within_bounds(&self, board: &ChessBoard) -> bool {
        let board_size = board.len() as i32; // Assuming a square board
        self.row_index >= 0
            && self.row_index < board_size
            && self.column_index >= 0
            && self.column_index < board_size
    }

    pub fn validate_move(&self, board: &ChessBoard) -> Option<BoardPosition> {
        if !self.is_position_within_bounds(board) {
            return None;
        }

        let space_is_unoccupied: bool = {
            let space: Option<&ChessPiece> =
                board[self.row_index as usize][self.column_index as usize].as_ref();
            space.is_none()
        };
        let valid_move: Option<BoardPosition> = if space_is_unoccupied {
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
        if !self.is_position_within_bounds(board) {
            return None;
        }

        let captured_piece: Option<&ChessPiece> =
            board[self.row_index as usize][self.column_index as usize].as_ref();
        let valid_capture = match captured_piece {
            Some(piece) if piece.get_player() != player => Some(BoardPosition {
                row_index: self.row_index as usize,
                column_index: self.column_index as usize,
            }),
            _ => None, // catch-all if captured_piece is None, or captured_piece is Some but piece.player() == player
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

#[cfg(test)]
mod tests {
    mod test_is_position_within_bounds {
        use crate::game::{
            board::{initialize_empty_board, ChessBoard},
            board_position::CandidateBoardPosition,
        };

        #[test]
        fn test_valid_row_and_valid_column_returns_true() {
            let board: ChessBoard = initialize_empty_board();
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 7,
                column_index: 0,
            };
            assert_eq!(position.is_position_within_bounds(&board), true);
        }

        #[test]
        fn test_valid_row_and_invalid_column_returns_false() {
            let board: ChessBoard = initialize_empty_board();
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 4,
                column_index: -1,
            };
            assert_eq!(position.is_position_within_bounds(&board), false);
        }

        #[test]
        fn test_invalid_row_and_valid_column_returns_false() {
            let board: ChessBoard = initialize_empty_board();
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 8,
                column_index: 3,
            };
            assert_eq!(position.is_position_within_bounds(&board), false);
        }

        #[test]
        fn test_invalid_row_and_invalid_column_returns_false() {
            let board: ChessBoard = initialize_empty_board();
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: -3,
                column_index: 10,
            };
            assert_eq!(position.is_position_within_bounds(&board), false);
        }
    }

    mod test_validate_move {
        use crate::{
            game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::{BoardPosition, CandidateBoardPosition},
            },
            piece::{pawn::Pawn, ChessPiece},
            player::Player,
        };

        #[test]
        fn test_invalid_position_returns_none() {
            let board: ChessBoard = initialize_empty_board();
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: -3,
                column_index: 10,
            };
            assert_eq!(position.validate_move(&board), None);
        }

        #[test]
        fn test_space_is_occupied_returns_none() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: BoardPosition {
                    row_index: 3,
                    column_index: 3,
                },
                has_been_moved: false,
            }));
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 3,
                column_index: 3,
            };
            assert_eq!(position.validate_move(&board), None);
        }

        #[test]
        fn test_space_is_unoccupied_returns_some_position() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: BoardPosition {
                    row_index: 3,
                    column_index: 3,
                },
                has_been_moved: false,
            }));
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 5,
                column_index: 5,
            };
            assert_eq!(
                position.validate_move(&board),
                Some(BoardPosition {
                    row_index: 5,
                    column_index: 5,
                })
            );
        }
    }

    mod test_validate_capture {
        use crate::{
            game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::{BoardPosition, CandidateBoardPosition},
            },
            piece::{pawn::Pawn, ChessPiece},
            player::Player,
        };

        #[test]
        fn test_invalid_position_returns_none() {
            let board: ChessBoard = initialize_empty_board();
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: -3,
                column_index: 10,
            };
            assert_eq!(position.validate_move(&board), None);
        }

        #[test]
        fn test_empty_position_returns_none() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: BoardPosition {
                    row_index: 3,
                    column_index: 3,
                },
                has_been_moved: false,
            }));
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 5,
                column_index: 5,
            };
            assert_eq!(position.validate_capture(&Player::White, &board), None);
        }

        #[test]
        fn test_same_player_occupied_position_returns_none() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: BoardPosition {
                    row_index: 3,
                    column_index: 3,
                },
                has_been_moved: false,
            }));
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 3,
                column_index: 3,
            };
            assert_eq!(position.validate_capture(&Player::Black, &board), None);
        }

        #[test]
        fn test_opposing_player_occupied_position_returns_some_position() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: BoardPosition {
                    row_index: 3,
                    column_index: 3,
                },
                has_been_moved: false,
            }));
            let position: CandidateBoardPosition = CandidateBoardPosition {
                row_index: 3,
                column_index: 3,
            };
            assert_eq!(
                position.validate_capture(&Player::White, &board),
                Some(BoardPosition {
                    row_index: 3,
                    column_index: 3,
                })
            );
        }
    }
}

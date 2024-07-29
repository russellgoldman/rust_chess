use crate::{
    piece::{ChessPiece, ChessPieceTrait},
    player::Player,
};

use super::board::ChessBoard;
use std::hash::Hash;

#[derive(Debug)]
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

    pub fn validate_candidate_position(&self, board: &ChessBoard) -> Option<BoardPosition> {
        // Perform validation code here
        let validated = self.is_position_within_bounds(board);

        if validated {
            Some(BoardPosition {
                row_index: self.row_index as usize,
                column_index: self.column_index as usize,
            })
        } else {
            None
        }
    }

    pub fn validate_candidate_position_and_unwrap(&self, board: &ChessBoard) -> BoardPosition {
        self.validate_candidate_position(&board).unwrap()
    }
}

/*
    BoardPosition fields are private. To obtain an instance of BoardPosition, create a CandidateBoardPosition
    instance and invoke `validate_capture_position` to receive an instance of Option<BoardPosition>

    Only use the `validate_capture_position_and_unwrap` method in cases where you are certain the position
    is valid and you want to directly receive an instance of BoardPosition. If the position is invalid,
    a panic will occur.
*/
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct BoardPosition {
    row_index: usize,
    column_index: usize,
}

impl BoardPosition {
    pub fn is_move_valid(&self, board: &ChessBoard) -> bool {
        let space_is_unoccupied: bool = {
            let space: Option<&ChessPiece> =
                board[self.row_index as usize][self.column_index as usize].as_ref();
            space.is_none()
        };
        space_is_unoccupied
    }

    pub fn is_capture_valid(&self, player: &Player, board: &ChessBoard) -> bool {
        let captured_piece: Option<&ChessPiece> =
            board[self.row_index as usize][self.column_index as usize].as_ref();
        // Returns true if captured_piece is Some and piece.player() != player, otherwise returns false
        captured_piece.is_some_and(|piece: &ChessPiece| piece.get_player() != player)
    }

    pub fn get_row_index(&self) -> usize {
        self.row_index
    }

    pub fn get_column_index(&self) -> usize {
        self.column_index
    }

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
    mod candidate_board_position {
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

        mod test_validate_candidate_position {
            use crate::game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::{BoardPosition, CandidateBoardPosition},
            };

            #[test]
            fn test_candidate_position_is_valid() {
                let position: CandidateBoardPosition = CandidateBoardPosition {
                    row_index: 0,
                    column_index: 7,
                };
                let board: ChessBoard = initialize_empty_board();
                assert_eq!(
                    position.validate_candidate_position(&board),
                    Some(BoardPosition {
                        row_index: 0,
                        column_index: 7
                    })
                );
            }

            #[test]
            fn test_candidate_position_is_invalid() {
                let position: CandidateBoardPosition = CandidateBoardPosition {
                    row_index: -1,
                    column_index: 7,
                };
                let board: ChessBoard = initialize_empty_board();
                assert_eq!(position.validate_candidate_position(&board), None);
            }
        }

        mod test_validate_candidate_position_and_unwrap {
            use crate::game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::{BoardPosition, CandidateBoardPosition},
            };

            #[test]
            fn test_candidate_position_is_valid() {
                let position: CandidateBoardPosition = CandidateBoardPosition {
                    row_index: 0,
                    column_index: 7,
                };
                let board: ChessBoard = initialize_empty_board();
                assert_eq!(
                    position.validate_candidate_position_and_unwrap(&board),
                    BoardPosition {
                        row_index: 0,
                        column_index: 7
                    }
                );
            }

            #[test]
            #[should_panic]
            fn test_candidate_position_is_invalid() {
                let position: CandidateBoardPosition = CandidateBoardPosition {
                    row_index: -1,
                    column_index: 7,
                };
                let board: ChessBoard = initialize_empty_board();
                position.validate_candidate_position_and_unwrap(&board);
            }
        }
    }

    mod board_position {
        mod test_is_move_valid {
            use crate::{
                game::{
                    board::{initialize_empty_board, ChessBoard},
                    board_position::{BoardPosition, CandidateBoardPosition},
                },
                piece::{pawn::Pawn, ChessPiece},
                player::Player,
            };

            #[test]
            fn test_space_is_occupied_returns_false() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][3] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: BoardPosition {
                        row_index: 3,
                        column_index: 3,
                    },
                    has_been_moved: false,
                }));
                let position: BoardPosition = CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board);
                assert_eq!(position.is_move_valid(&board), false);
            }

            #[test]
            fn test_space_is_unoccupied_returns_true() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][5] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: BoardPosition {
                        row_index: 3,
                        column_index: 5,
                    },
                    has_been_moved: false,
                }));
                let position: BoardPosition = CandidateBoardPosition {
                    row_index: 5,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board);
                assert_eq!(position.is_move_valid(&board), true);
            }
        }

        mod test_is_capture_valid {
            use crate::{
                game::{
                    board::{initialize_empty_board, ChessBoard},
                    board_position::{BoardPosition, CandidateBoardPosition},
                },
                piece::{pawn::Pawn, ChessPiece},
                player::Player,
            };

            #[test]
            fn test_empty_position_returns_false() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][3] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: BoardPosition {
                        row_index: 3,
                        column_index: 3,
                    },
                    has_been_moved: false,
                }));
                let position: BoardPosition = CandidateBoardPosition {
                    row_index: 5,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board);
                assert_eq!(position.is_capture_valid(&Player::White, &board), false);
            }

            #[test]
            fn test_same_player_occupied_position_returns_false() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][3] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: BoardPosition {
                        row_index: 3,
                        column_index: 3,
                    },
                    has_been_moved: false,
                }));
                let position: BoardPosition = CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board);
                assert_eq!(position.is_capture_valid(&Player::Black, &board), false);
            }

            #[test]
            fn test_opposing_player_occupied_position_returns_true() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][3] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: BoardPosition {
                        row_index: 3,
                        column_index: 3,
                    },
                    has_been_moved: false,
                }));
                let position: BoardPosition = CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board);
                assert_eq!(position.is_capture_valid(&Player::White, &board), true);
            }
        }
    }
}

use crate::{
    game::{board::ChessBoard, board_position::CandidateBoardPosition},
    piece::{BoardPosition, ChessPieceTrait, PieceMoveData},
    player::Player,
};

#[derive(Debug)]
pub struct Bishop {
    pub player: Player,
    pub position: BoardPosition,
}

impl ChessPieceTrait for Bishop {
    fn get_player(&self) -> &Player {
        &self.player
    }

    fn valid_moves_and_captures(&self, board: &ChessBoard) -> PieceMoveData {
        /*
            Both Black and White Bishops can move up and down diagonally across the board.
            As such, we do not need to hardcode a direction variable as we did with Pawns.

            We need to check for all diagonal positions that a bishop can move. A bishop
            cannot jump over a piece, so the first piece in the diagonal left or right
            path that a bishop encounters will stop the diagonal movement possibilities.
        */

        let mut valid_moves: Vec<BoardPosition> = vec![];
        let valid_captures: Vec<BoardPosition> = vec![];
        let directions: [(i32, i32); 4] = [
            (1, -1),  // upward-left diagonal
            (1, 1),   // upward-right diagonal
            (-1, -1), // downward-left diagonal
            (-1, 1),  // down-wardright diagonal
        ];

        for (row_offset, column_offset) in directions.iter() {
            let mut distance: i32 = 1;
            loop {
                let position: CandidateBoardPosition = CandidateBoardPosition {
                    row_index: self.position.get_row_index() as i32 + *row_offset * distance,
                    column_index: self.position.get_column_index() as i32
                        + *column_offset * distance,
                };

                if let Some(position) = position
                    .validate_candidate_position(&board)
                    .filter(|position| position.is_move_valid(&board))
                {
                    valid_moves.push(position);
                    distance += 1;
                } else {
                    break;
                }
            }
        }

        PieceMoveData {
            valid_moves,
            valid_captures,
        }
    }

    fn display_piece_on_board(&self) {
        match self.player {
            Player::Black => print!("(B)"),
            Player::White => print!("[B]"),
        }
    }

    fn get_piece_name(&self) -> &str {
        "Bishop"
    }

    fn get_board_position(&self) -> &BoardPosition {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    mod test_get_player {
        use crate::{
            game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::CandidateBoardPosition,
            },
            piece::{bishop::Bishop, ChessPieceTrait},
            player::Player,
        };

        #[test]
        fn test_get_black_player() {
            let board: ChessBoard = initialize_empty_board();
            let bishop = Bishop {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
            };
            assert_eq!(bishop.get_player(), &Player::Black);
        }

        #[test]
        fn test_get_white_player() {
            let board: ChessBoard = initialize_empty_board();
            let pawn = Bishop {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
            };
            assert_eq!(pawn.get_player(), &Player::White);
        }
    }

    /*
        We will create sub-modules to individually test bishop
        move and capture logic independently. Unit tests containing
        both bishop moves and captures will be included in the
        `test_valid_moves_and_captures` module.
    */
    mod test_valid_moves_and_captures {
        /*
            Include unit tests for both bishop moves and captures
            here ...
        */

        mod test_valid_moves {
            use crate::{
                game::{
                    board::{initialize_empty_board, ChessBoard},
                    board_position::CandidateBoardPosition,
                },
                piece::{bishop::Bishop, pawn::Pawn, ChessPiece, ChessPieceTrait, PieceMoveData},
                player::Player,
            };

            #[test]
            fn test_upward_left_diagonal_moves_are_blocked() {
                let mut board: ChessBoard = initialize_empty_board();
                board[1][6] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 1,
                        column_index: 6,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_left_diagonal_move_is_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[2][5] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 2,
                        column_index: 5,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![CandidateBoardPosition {
                            row_index: 1,
                            column_index: 6,
                        }
                        .validate_candidate_position_and_unwrap(&board)],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_left_diagonal_moves_are_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][4] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 3,
                        column_index: 4,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![
                            CandidateBoardPosition {
                                row_index: 1,
                                column_index: 6,
                            }
                            .validate_candidate_position_and_unwrap(&board),
                            CandidateBoardPosition {
                                row_index: 2,
                                column_index: 5,
                            }
                            .validate_candidate_position_and_unwrap(&board)
                        ],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_left_diagonal_invalid_move_is_ignored() {
                let mut board: ChessBoard = initialize_empty_board();
                board[6][1] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 6,
                        column_index: 1,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_right_diagonal_moves_are_blocked() {
                let mut board: ChessBoard = initialize_empty_board();
                board[1][1] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 1,
                        column_index: 1,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_right_diagonal_move_is_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[2][2] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 2,
                        column_index: 2,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![CandidateBoardPosition {
                            row_index: 1,
                            column_index: 1,
                        }
                        .validate_candidate_position_and_unwrap(&board)],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_right_diagonal_moves_are_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[3][3] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 3,
                        column_index: 3,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![
                            CandidateBoardPosition {
                                row_index: 1,
                                column_index: 1,
                            }
                            .validate_candidate_position_and_unwrap(&board),
                            CandidateBoardPosition {
                                row_index: 2,
                                column_index: 2,
                            }
                            .validate_candidate_position_and_unwrap(&board)
                        ],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_right_diagonal_invalid_move_is_ignored() {
                let mut board: ChessBoard = initialize_empty_board();
                board[6][6] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 6,
                        column_index: 6,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_left_diagonal_moves_are_blocked() {
                let mut board: ChessBoard = initialize_empty_board();
                board[6][6] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 6,
                        column_index: 6,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_left_diagonal_move_is_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[5][5] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 5,
                        column_index: 5,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![CandidateBoardPosition {
                            row_index: 6,
                            column_index: 6,
                        }
                        .validate_candidate_position_and_unwrap(&board)],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_left_diagonal_moves_are_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[4][4] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 4,
                        column_index: 4,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![
                            CandidateBoardPosition {
                                row_index: 6,
                                column_index: 6,
                            }
                            .validate_candidate_position_and_unwrap(&board),
                            CandidateBoardPosition {
                                row_index: 5,
                                column_index: 5,
                            }
                            .validate_candidate_position_and_unwrap(&board)
                        ],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_left_diagonal_invalid_move_is_ignored() {
                let mut board: ChessBoard = initialize_empty_board();
                board[1][1] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 1,
                        column_index: 1,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_right_diagonal_moves_are_blocked() {
                let mut board: ChessBoard = initialize_empty_board();
                board[6][1] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 6,
                        column_index: 1,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_right_diagonal_move_is_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[5][2] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 5,
                        column_index: 2,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![CandidateBoardPosition {
                            row_index: 6,
                            column_index: 1,
                        }
                        .validate_candidate_position_and_unwrap(&board)],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_right_diagonal_moves_are_available() {
                let mut board: ChessBoard = initialize_empty_board();
                board[4][3] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 4,
                        column_index: 3,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 7,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![
                            CandidateBoardPosition {
                                row_index: 6,
                                column_index: 1,
                            }
                            .validate_candidate_position_and_unwrap(&board),
                            CandidateBoardPosition {
                                row_index: 5,
                                column_index: 2,
                            }
                            .validate_candidate_position_and_unwrap(&board)
                        ],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_downward_right_diagonal_invalid_move_is_ignored() {
                let mut board: ChessBoard = initialize_empty_board();
                board[1][6] = Some(ChessPiece::Pawn(Pawn {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 1,
                        column_index: 6,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    has_been_moved: false,
                }));
                let bishop: ChessPiece = ChessPiece::Bishop(Bishop {
                    player: Player::Black,
                    position: CandidateBoardPosition {
                        row_index: 0,
                        column_index: 7,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                });
                assert_eq!(
                    bishop.valid_moves_and_captures(&board),
                    PieceMoveData {
                        valid_moves: vec![],
                        valid_captures: vec![],
                    }
                );
            }

            #[test]
            fn test_upward_left_and_upward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_left_and_downward_left_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_left_and_downward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_right_and_downward_left_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_right_and_downward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_downward_left_and_downward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_left_and_upward_right_and_downward_left_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_left_and_upward_right_and_downward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_left_and_downward_left_and_downward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_upward_right_and_downward_left_and_downward_right_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_all_direction_moves_are_available() {
                todo!();
            }

            #[test]
            fn test_all_direction_no_moves_are_available() {
                todo!();
            }
        }
    }
}

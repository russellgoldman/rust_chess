use crate::{
    game::{board::ChessBoard, board_position::CandidateBoardPosition},
    piece::{BoardPosition, ChessPieceTrait, PieceMoveData},
    player::Player,
};

// NOTE: We will not be implementing Pawn promotion or the En Passant capture rule for simplicity
#[derive(Debug)]
pub struct Pawn {
    pub player: Player,
    pub position: BoardPosition,
    pub has_been_moved: bool,
}

impl ChessPieceTrait for Pawn {
    fn get_player(&self) -> &Player {
        &self.player
    }

    fn valid_moves_and_captures(&self, board: &ChessBoard) -> PieceMoveData {
        PieceMoveData {
            valid_moves: self.valid_moves(&board),
            valid_captures: self.valid_captures(&board),
        }
    }

    fn display_piece_on_board(&self) {
        match self.player {
            Player::Black => print!("(P)"),
            Player::White => print!("[P]"),
        }
    }

    fn get_piece_name(&self) -> &str {
        "Pawn"
    }

    fn get_board_position(&self) -> &BoardPosition {
        &self.position
    }
}

impl Pawn {
    fn valid_moves(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        let mut valid_moves: Vec<BoardPosition> = vec![];
        let pawn_direction: i32 = match self.player {
            Player::White => 1,
            Player::Black => -1,
        };

        let normal_move: CandidateBoardPosition = CandidateBoardPosition {
            row_index: self.position.get_row_index() as i32 + pawn_direction,
            column_index: self.position.get_column_index() as i32,
        };

        if let Some(normal_move_position) = normal_move
            .validate_candidate_position(&board)
            .filter(|position| position.is_move_valid(&board))
        {
            valid_moves.push(normal_move_position);

            /*
                We have decided to include the two position move check within the one position move check.
                The reason being is that a pawn cannot jump over an opposing piece, so if a pawn cannot be moved one position
                forward, it also cannot be moved two positions forward.
            */
            if !self.has_been_moved {
                let initial_two_move: CandidateBoardPosition = CandidateBoardPosition {
                    row_index: self.position.get_row_index() as i32 + (pawn_direction * 2),
                    column_index: self.position.get_column_index() as i32,
                };

                if let Some(initial_two_move_position) = initial_two_move
                    .validate_candidate_position(&board)
                    .filter(|position| position.is_move_valid(&board))
                {
                    valid_moves.push(initial_two_move_position);
                }
            }
        }

        valid_moves
    }

    fn valid_captures(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        let mut valid_captures: Vec<BoardPosition> = vec![];
        let pawn_direction: i32 = match self.player {
            Player::White => 1,
            Player::Black => -1,
        };

        let left_diagonal_capture: CandidateBoardPosition = CandidateBoardPosition {
            row_index: self.position.get_row_index() as i32 + (pawn_direction),
            column_index: self.position.get_column_index() as i32 - 1,
        };

        if let Some(board_position) = left_diagonal_capture
            .validate_candidate_position(&board)
            .filter(|position| position.is_capture_valid(&self.player, &board))
        {
            valid_captures.push(board_position);
        }

        let right_diagonal_capture: CandidateBoardPosition = CandidateBoardPosition {
            row_index: self.position.get_row_index() as i32 + (pawn_direction),
            column_index: self.position.get_column_index() as i32 + 1,
        };
        if let Some(board_position) = right_diagonal_capture
            .validate_candidate_position(&board)
            .filter(|position| position.is_capture_valid(&self.player, &board))
        {
            valid_captures.push(board_position);
        }

        valid_captures
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
            piece::{pawn::Pawn, ChessPieceTrait},
            player::Player,
        };

        #[test]
        fn test_get_black_player() {
            let board: ChessBoard = initialize_empty_board();
            let pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(pawn.get_player(), &Player::Black);
        }

        #[test]
        fn test_get_white_player() {
            let board: ChessBoard = initialize_empty_board();
            let pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(pawn.get_player(), &Player::White);
        }
    }

    mod test_valid_moves_and_captures {
        use crate::{
            game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::CandidateBoardPosition,
            },
            piece::{pawn::Pawn, ChessPiece, ChessPieceTrait, PieceMoveData},
            player::Player,
        };

        #[test]
        fn test_valid_moves_and_no_captures() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: ChessPiece = ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            });
            assert_eq!(
                pawn.valid_moves_and_captures(&board),
                PieceMoveData {
                    valid_moves: vec![CandidateBoardPosition {
                        row_index: 4,
                        column_index: 5
                    }
                    .validate_candidate_position_and_unwrap(&board),],
                    valid_captures: vec![]
                }
            );
        }

        #[test]
        fn test_valid_captures_and_no_moves() {
            let mut board: ChessBoard = initialize_empty_board();
            board[4][4] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            board[4][5] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: ChessPiece = ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            });
            assert_eq!(
                pawn.valid_moves_and_captures(&board),
                PieceMoveData {
                    valid_moves: vec![],
                    valid_captures: vec![CandidateBoardPosition {
                        row_index: 4,
                        column_index: 4
                    }
                    .validate_candidate_position_and_unwrap(&board),]
                }
            );
        }

        #[test]
        fn test_valid_moves_and_captures() {
            let mut board: ChessBoard = initialize_empty_board();
            board[4][4] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            board[4][6] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 6,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: ChessPiece = ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            });
            assert_eq!(
                pawn.valid_moves_and_captures(&board),
                PieceMoveData {
                    valid_moves: vec![CandidateBoardPosition {
                        row_index: 4,
                        column_index: 5
                    }
                    .validate_candidate_position_and_unwrap(&board),],
                    valid_captures: vec![
                        CandidateBoardPosition {
                            row_index: 4,
                            column_index: 4
                        }
                        .validate_candidate_position_and_unwrap(&board),
                        CandidateBoardPosition {
                            row_index: 4,
                            column_index: 6
                        }
                        .validate_candidate_position_and_unwrap(&board),
                    ]
                }
            );
        }

        #[test]
        fn test_no_valid_moves_or_captures() {
            let mut board: ChessBoard = initialize_empty_board();
            board[4][5] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: ChessPiece = ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            });
            assert_eq!(
                pawn.valid_moves_and_captures(&board),
                PieceMoveData {
                    valid_moves: vec![],
                    valid_captures: vec![],
                }
            );
        }
    }

    mod test_valid_moves {
        use crate::{
            game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::CandidateBoardPosition,
            },
            piece::{pawn::Pawn, ChessPiece},
            player::Player,
        };

        #[test]
        fn test_black_only_normal_move_is_valid_if_pawn_has_been_moved() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_moves(&board),
                vec![CandidateBoardPosition {
                    row_index: 3,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),]
            );
        }

        #[test]
        fn test_black_only_normal_move_is_valid_if_piece_is_blocking() {
            let mut board: ChessBoard = initialize_empty_board();
            board[5][0] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 7,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(
                pawn.valid_moves(&board),
                vec![CandidateBoardPosition {
                    row_index: 6,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),]
            );
        }

        #[test]
        fn test_black_normal_and_initial_moves_are_valid() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 7,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(
                pawn.valid_moves(&board),
                vec![
                    CandidateBoardPosition {
                        row_index: 6,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    CandidateBoardPosition {
                        row_index: 5,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board)
                ]
            );
        }

        #[test]
        fn test_black_no_valid_moves() {
            let mut board: ChessBoard = initialize_empty_board();
            board[6][0] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 6,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 7,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(pawn.valid_moves(&board), vec![]);
        }

        #[test]
        fn test_black_invalid_move_position_is_ignored() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 0,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_moves(&board), vec![]);
        }

        #[test]
        fn test_white_only_normal_move_is_valid_if_pawn_has_been_moved() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_moves(&board),
                vec![CandidateBoardPosition {
                    row_index: 4,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board)]
            )
        }

        #[test]
        fn test_white_only_normal_move_is_valid_if_piece_is_blocking() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][0] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(
                pawn.valid_moves(&board),
                vec![CandidateBoardPosition {
                    row_index: 2,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board)]
            );
        }

        #[test]
        fn test_white_normal_and_initial_moves_are_valid() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(
                pawn.valid_moves(&board),
                vec![
                    CandidateBoardPosition {
                        row_index: 2,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    CandidateBoardPosition {
                        row_index: 3,
                        column_index: 0,
                    }
                    .validate_candidate_position_and_unwrap(&board)
                ]
            );
        }

        #[test]
        fn test_white_no_valid_moves() {
            let mut board: ChessBoard = initialize_empty_board();
            board[2][0] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 2,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 1,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: false,
            };
            assert_eq!(pawn.valid_moves(&board), vec![]);
        }

        #[test]
        fn test_white_invalid_move_position_is_ignored() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 7,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_moves(&board), vec![]);
        }
    }

    mod test_valid_captures {
        use crate::{
            game::{
                board::{initialize_empty_board, ChessBoard},
                board_position::CandidateBoardPosition,
            },
            piece::{pawn::Pawn, ChessPiece},
            player::Player,
        };

        #[test]
        fn test_black_left_diagonal_capture_is_valid() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_captures(&board),
                vec![CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board)]
            );
        }

        #[test]
        fn test_black_right_diagonal_capture_is_valid() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][5] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_captures(&board),
                vec![CandidateBoardPosition {
                    row_index: 3,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board)]
            );
        }

        #[test]
        fn test_black_left_and_right_diagonal_captures_are_valid() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            board[3][5] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_captures(&board),
                vec![
                    CandidateBoardPosition {
                        row_index: 3,
                        column_index: 3,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    CandidateBoardPosition {
                        row_index: 3,
                        column_index: 5,
                    }
                    .validate_candidate_position_and_unwrap(&board)
                ]
            )
        }

        #[test]
        fn test_black_no_valid_captures() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_captures(&board), vec![]);
        }

        #[test]
        fn test_black_same_player_diagonal_capture_is_ignored() {
            let mut board: ChessBoard = initialize_empty_board();
            board[3][3] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 3,
                    column_index: 3,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_captures(&board), vec![]);
        }

        #[test]
        fn test_black_invalid_capture_position_is_ignored() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 0,
                    column_index: 0,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_captures(&board), vec![])
        }

        #[test]
        fn test_white_left_diagonal_capture_is_valid() {
            let mut board: ChessBoard = initialize_empty_board();
            board[5][4] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_captures(&board),
                vec![CandidateBoardPosition {
                    row_index: 5,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board)]
            );
        }

        #[test]
        fn test_white_right_diagonal_capture_is_valid() {
            let mut board: ChessBoard = initialize_empty_board();
            board[5][6] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 6,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_captures(&board),
                vec![CandidateBoardPosition {
                    row_index: 5,
                    column_index: 6,
                }
                .validate_candidate_position_and_unwrap(&board)]
            );
        }

        #[test]
        fn test_white_left_and_right_diagonal_captures_are_valid() {
            let mut board: ChessBoard = initialize_empty_board();
            board[5][4] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            board[5][6] = Some(ChessPiece::Pawn(Pawn {
                player: Player::Black,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 6,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(
                pawn.valid_captures(&board),
                vec![
                    CandidateBoardPosition {
                        row_index: 5,
                        column_index: 4,
                    }
                    .validate_candidate_position_and_unwrap(&board),
                    CandidateBoardPosition {
                        row_index: 5,
                        column_index: 6,
                    }
                    .validate_candidate_position_and_unwrap(&board)
                ]
            )
        }

        #[test]
        fn test_white_no_valid_captures() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_captures(&board), vec![]);
        }

        #[test]
        fn test_white_same_player_diagonal_capture_is_ignored() {
            let mut board: ChessBoard = initialize_empty_board();
            board[5][4] = Some(ChessPiece::Pawn(Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 5,
                    column_index: 4,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            }));
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 4,
                    column_index: 5,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_captures(&board), vec![]);
        }

        #[test]
        fn test_white_invalid_capture_position_is_ignored() {
            let board: ChessBoard = initialize_empty_board();
            let pawn: Pawn = Pawn {
                player: Player::White,
                position: CandidateBoardPosition {
                    row_index: 7,
                    column_index: 7,
                }
                .validate_candidate_position_and_unwrap(&board),
                has_been_moved: true,
            };
            assert_eq!(pawn.valid_captures(&board), vec![])
        }
    }
}

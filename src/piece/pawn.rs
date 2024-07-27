use crate::{
    game::{board::ChessBoard, board_position::CandidateBoardPosition},
    piece::{BoardPosition, ChessPieceTrait},
    player::Player,
};

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

    fn valid_moves(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        /*
            Look at the current position of the piece

            - Pawn's can move one or two spaces forward only if they haven't been moved in the game yet
            - Otherwise, Pawns' can only move one space forward
            - White Pawn's can only move forward in the range of rows 1 - 7 (increasing)
            - Black Pawn's can only move forward in the range of rows 6 - 0 (decreasing)
        */
        let mut valid_moves: Vec<BoardPosition> = vec![];
        let pawn_direction: i32 = match self.player {
            Player::White => 1,
            Player::Black => -1,
        };

        if !self.has_been_moved {
            let initial_two_move = CandidateBoardPosition {
                row_index: self.position.row_index as i32 + (pawn_direction * 2),
                column_index: self.position.column_index as i32,
            };
            if let Some(board_position) = initial_two_move.validate_move(board) {
                valid_moves.push(board_position);
            }
        }
        let normal_move = CandidateBoardPosition {
            row_index: self.position.row_index as i32 + pawn_direction,
            column_index: self.position.column_index as i32,
        };
        if let Some(board_position) = normal_move.validate_move(board) {
            valid_moves.push(board_position);
        }

        valid_moves
    }

    fn valid_captures(&self, board: &ChessBoard) -> Vec<BoardPosition> {
        /*
            Look at the current position of the piece
            - Pawn's can capture enemy pieces by moving one space forward diagonally

            For simplicity, we will not be implementing the En Passant rule.
        */
        let mut valid_captures: Vec<BoardPosition> = vec![];
        let pawn_direction: i32 = match self.player {
            Player::White => 1,
            Player::Black => -1,
        };

        let left_diagonal_capture = CandidateBoardPosition {
            row_index: self.position.row_index as i32 + (pawn_direction),
            column_index: self.position.column_index as i32 - 1,
        };
        if let Some(board_position) = left_diagonal_capture.validate_capture(&self.player, board) {
            valid_captures.push(board_position);
        }

        let right_diagonal_capture = CandidateBoardPosition {
            row_index: self.position.row_index as i32 + (pawn_direction),
            column_index: self.position.column_index as i32 + 1,
        };
        if let Some(board_position) = right_diagonal_capture.validate_capture(&self.player, board) {
            valid_captures.push(board_position);
        }

        valid_captures
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

#[cfg(test)]
mod tests {
    mod test_get_player {
        use crate::{
            game::board_position::BoardPosition,
            piece::{pawn::Pawn, ChessPieceTrait},
            player::Player,
        };

        #[test]
        fn test_get_black_player() {
            let pawn = Pawn {
                player: Player::Black,
                position: BoardPosition {
                    row_index: 1,
                    column_index: 0,
                },
                has_been_moved: false,
            };
            assert_eq!(pawn.get_player(), &Player::Black);
        }

        #[test]
        fn test_get_white_player() {
            let pawn = Pawn {
                player: Player::White,
                position: BoardPosition {
                    row_index: 1,
                    column_index: 0,
                },
                has_been_moved: false,
            };
            assert_eq!(pawn.get_player(), &Player::White);
        }
    }

    mod test_valid_moves {
        #[test]
        fn test_black_normal_move_is_valid() {
            todo!();
        }

        #[test]
        fn test_black_initial_move_is_valid() {
            todo!();
        }

        #[test]
        fn test_black_normal_and_initial_moves_are_valid() {
            todo!();
        }

        #[test]
        fn test_black_no_valid_moves() {
            todo!();
        }

        #[test]
        fn test_black_invalid_move_position_not_included() {
            todo!();
        }

        #[test]
        fn test_white_normal_move_is_valid() {
            todo!();
        }

        #[test]
        fn test_white_initial_move_is_valid() {
            todo!();
        }

        #[test]
        fn test_white_normal_and_initial_moves_are_valid() {
            todo!();
        }

        #[test]
        fn test_white_no_valid_moves() {
            todo!();
        }

        #[test]
        fn test_white_invalid_move_position_not_included() {
            todo!();
        }
    }

    mod test_valid_captures {
        #[test]
        fn test_black_left_diagonal_capture_is_valid() {
            todo!();
        }

        #[test]
        fn test_black_right_diagonal_capture_is_valid() {
            todo!();
        }

        #[test]
        fn test_black_left_and_right_diagonal_captures_are_valid() {
            todo!();
        }

        #[test]
        fn test_black_no_valid_captures() {
            todo!();
        }

        #[test]
        fn test_black_invalid_capture_position_not_included() {
            todo!();
        }

        #[test]
        fn test_white_left_diagonal_capture_is_valid() {
            todo!();
        }

        #[test]
        fn test_white_right_diagonal_capture_is_valid() {
            todo!();
        }

        #[test]
        fn test_white_left_and_right_diagonal_captures_are_valid() {
            todo!();
        }

        #[test]
        fn test_white_no_valid_captures() {
            todo!();
        }

        #[test]
        fn test_white_invalid_capture_position_not_included() {
            todo!();
        }
    }
}

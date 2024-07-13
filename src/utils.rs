use crate::{
    board::ChessBoard,
    piece::{
        bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook,
        ChessPiece,
    },
    player::Player,
};

// Chess board should be an 8x8 2d vector
pub fn initialize_chess_board() -> ChessBoard {
    let mut board: Vec<Vec<Option<ChessPiece>>> = vec![];
    for row_index in 0..=7 {
        let mut board_row: Vec<Option<ChessPiece>> = vec![];
        for column_index in 0..=7 {
            if row_index >= 2 && row_index <= 5 {
                // Non-occupied space (row indices 2, 3, 4, 5)
                board_row.push(None);
                continue;
            } else {
                let player: Player = match row_index {
                    // White Player start occupied space (row indices 0, 1)
                    0 | 1 => Player::White,
                    // Black Player start occupied space (row indices 6, 7)
                    6 | 7 => Player::Black,
                    _ => panic!("Given row_index: {row_index}. This should not be evaluated as a player starting row index.")
                };

                // Player occupied space (row indices 0, 1, 6, 7)
                let piece: Option<ChessPiece> = match row_index {
                    0 | 7 => match column_index {
                        0 | 7 => Some(ChessPiece::Rook(Rook { player })),
                        1 | 6 => Some(ChessPiece::Knight(Knight { player })),
                        2 | 5 => Some(ChessPiece::Bishop(Bishop { player })),
                        3 => Some(ChessPiece::Queen(Queen { player })),
                        4 => Some(ChessPiece::King(King { player })),
                        _ => panic!("Invalid {column_index}"),
                    },
                    1 | 6 => Some(ChessPiece::Pawn(Pawn { player, has_been_moved: false })),
                    _ => panic!("Given row_index: {row_index}. This should not be evaluated as a starting row index.")
                };

                board_row.push(piece);
            }
        }
        board.push(board_row)
    }

    ChessBoard { board }
}

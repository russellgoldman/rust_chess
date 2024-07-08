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
    let mut board: ChessBoard = vec![];
    for row_index in 0..=7 {
        let mut board_row: Vec<Option<ChessPiece>> = vec![];
        for column_index in 0..=7 {
            if row_index >= 2 && row_index <= 5 {
                // Non-occupied space (row indices 2, 3, 4, 5)
                board_row.push(Option::None);
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
                        0 | 7 => Option::Some(ChessPiece::Rook(Rook { player })),
                        1 | 6 => Option::Some(ChessPiece::Knight(Knight { player })),
                        2 | 5 => Option::Some(ChessPiece::Bishop(Bishop { player })),
                        3 => Option::Some(ChessPiece::Queen(Queen { player })),
                        4 => Option::Some(ChessPiece::King(King { player })),
                        _ => panic!("Invalid {column_index}"),
                    },
                    1 | 6 => Option::Some(ChessPiece::Pawn(Pawn { player, has_been_moved: false })),  // Pawn
                    _ => panic!("Given row_index: {row_index}. This should not be evaluated as a starting row index.")
                };

                board_row.push(piece);
            }
        }
        board.push(board_row)
    }

    board
}

// /*
//     TODO:

//     Given a position for a chess piece and an immutable reference to the board,
//     this function determines all valid moves this chess piece can make. This function
//     returns a vector of all valid moves.
// */
// fn valid_moves(piece_position: &BoardPosition, board: &ChessBoard) -> Vec<BoardPosition> {
//     let row_index = piece_position.row_index;
//     let column_index = piece_position.column_index;
//     let chess_piece = match board[row_index as usize][column_index as usize] {
//         Some(piece) => piece,
//         None => panic!("No chess piece found at piece_position: {piece_position:?}. Cannot determine valid moves for an unoccupied space."),
//     };

//     let valid_moves = match chess_piece.piece_type {
//         ChessPieceType::Pawn(pawn_data) => match chess_piece.piece_player {
//             Player::Black => valid_black_pawn_moves(&chess_piece, piece_position, pawn_data, board),
//             Player::White => valid_white_pawn_moves(&chess_piece, piece_position, pawn_data, board),
//         },
//         _ => panic!("TODO: Please implement valid moves function for piece type"),
//     };
// }

// fn board_position_is_within_bounds(position: &BoardPosition) -> bool {}

mod game;
mod piece;
mod player;

use game::ChessGame;
use inquire::Select;
use piece::{ChessPiece, ChessPieceTrait};
use player::Player;

fn main() {
    // Initialize ChessGame
    let mut game = ChessGame::new();
    game.display_board();

    loop {
        let players_turn: Player = if game.turn % 2 == 0 {
            Player::White
        } else {
            Player::Black
        };
        println!("Turn {}: Player {}'s Turn", game.turn + 1, players_turn);
        match players_turn {
            Player::White => {
                let available_pieces: Vec<&ChessPiece> =
                    game.board_data.white_pieces.values().collect();

                let piece: &ChessPiece =
                    Select::new("Please select a piece to move:", available_pieces)
                        .prompt()
                        .unwrap_or_else(|err| {
                            panic!(
                                "Failed to select a piece due to an Inquire error: {:?}",
                                err
                            )
                        });

                let valid_moves = piece.valid_moves(&game.board_data.board);
                let valid_captures = piece.valid_captures(&game.board_data.board);
            }
            Player::Black => {
                println!("TODO");
            }
        }

        game.turn += 1;
        if game.turn == 3 {
            break;
        }
    }
}

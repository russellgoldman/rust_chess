mod game;
mod piece;
mod player;

use game::ChessGame;
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
                println!("TODO");
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

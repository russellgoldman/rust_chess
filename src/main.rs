mod board;
mod piece;
mod player;
mod utils;

use utils::initialize_chess_board;

fn main() {
    // Initialize the board
    let chess_board = initialize_chess_board();
    chess_board.display_board();
}

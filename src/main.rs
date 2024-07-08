mod board;
mod piece;
mod player;
mod utils;

use utils::initialize_chess_board;

fn main() {
    // Initialize the board
    let board = initialize_chess_board();
    print!("{:#?}", board);
}

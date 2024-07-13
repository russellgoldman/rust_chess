use crate::piece::{ChessPiece, ChessPieceTrait};

#[derive(Debug)]
pub struct BoardPosition {
    pub row_index: i32,
    pub column_index: i32,
}
pub struct ChessBoard {
    pub board: Vec<Vec<Option<ChessPiece>>>,
}

impl ChessBoard {
    pub fn display_board(&self) {
        println!();
        println!("{}[Black]{}", " ".repeat(20), " ".repeat(20));
        println!("{}", "-".repeat(49));
        for row in self.board.iter() {
            print!("|");
            for piece in row.iter() {
                print!(" ");
                match piece {
                    Some(piece) => piece.print_piece(),
                    None => print!("   "),
                }
                print!(" |");
            }
            println!();
            println!("{}", "-".repeat(49));
        }
        println!("{}(White){}", " ".repeat(20), " ".repeat(20));
    }
}

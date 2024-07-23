use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
        }
    }
}

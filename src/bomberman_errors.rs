use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum BombermanError {
    MazeNotSquare(String),
    InvalidSquare(String),
    NoBombInStartingPosition(String),
}

impl Display for BombermanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BombermanError::MazeNotSquare(e) => write!(f, "MazeNotSquare: {e}"),
            BombermanError::InvalidSquare(e) => write!(f, "InvalidSquare: {e}"),
            BombermanError::NoBombInStartingPosition(e) => {
                write!(f, "NoBombInStartingPosition: {e}")
            }
        }
    }
}

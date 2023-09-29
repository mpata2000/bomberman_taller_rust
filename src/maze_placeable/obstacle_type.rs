use std::fmt::Display;

pub const WALL: &str = "W";
pub const ROCK: &str = "R";
pub const REDIRECTION: &str = "D";
pub const REDIRECTION_UP: &str = "DU";
pub const REDIRECTION_DOWN: &str = "DD";
pub const REDIRECTION_LEFT: &str = "DL";
pub const REDIRECTION_RIGHT: &str = "DR";

#[derive(Debug, PartialEq)]
pub enum ObstacleType {
    Wall,
    Rock,
    RedirectionUp,
    RedirectionDown,
    RedirectionLeft,
    RedirectionRight,
}

impl ObstacleType {
    pub fn new(square: &str) -> Result<ObstacleType, String> {
        match square {
            WALL => Ok(ObstacleType::Wall),
            ROCK => Ok(ObstacleType::Rock),
            REDIRECTION_UP => Ok(ObstacleType::RedirectionUp),
            REDIRECTION_DOWN => Ok(ObstacleType::RedirectionDown),
            REDIRECTION_LEFT => Ok(ObstacleType::RedirectionLeft),
            REDIRECTION_RIGHT => Ok(ObstacleType::RedirectionRight),
            _ => Err("invalid obstacle".to_string()),
        }
    }

    // Return true if the square is an obstacle
    pub fn is_obstacle(square: &str) -> bool {
        square == WALL || square == ROCK || square.starts_with(REDIRECTION)
    }
}

impl Display for ObstacleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObstacleType::Wall => write!(f, "{WALL}"),
            ObstacleType::Rock => write!(f, "{ROCK}"),
            ObstacleType::RedirectionUp => write!(f, "{REDIRECTION_UP}"),
            ObstacleType::RedirectionDown => write!(f, "{REDIRECTION_DOWN}"),
            ObstacleType::RedirectionLeft => write!(f, "{REDIRECTION_LEFT}"),
            ObstacleType::RedirectionRight => write!(f, "{REDIRECTION_RIGHT}"),
        }
    }
}

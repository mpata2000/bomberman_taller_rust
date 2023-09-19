use std::fmt::Display;
use crate::bomb::BombType;
use crate::bomberman::BombermanError::InvalidSquare;
use crate::bomberman::{BombermanError, MazeDisplay};
use crate::point::{Direction, Point};

pub const WALL: &str = "W";
pub const ROCK: &str = "R";
pub const  REDIRECTION: &str = "D";
pub const REDIRECTION_UP: &str = "DU";
pub const REDIRECTION_DOWN: &str = "DD";
pub const REDIRECTION_LEFT: &str = "DL";
pub const REDIRECTION_RIGHT: &str = "DR";

// Return true if the square is an obstacle
pub fn is_obstacle(square: &str) -> bool {
    square == WALL || square == ROCK || square.starts_with(REDIRECTION)
}

#[derive(Debug, PartialEq)]
pub(crate) enum ObstacleType {
    Wall,
    Rock,
    RedirectionUp,
    RedirectionDown,
    RedirectionLeft,
    RedirectionRight,
}

impl Display for ObstacleType{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObstacleType::Wall => write!(f, "{}", WALL),
            ObstacleType::Rock => write!(f, "{}", ROCK),
            ObstacleType::RedirectionUp => write!(f, "{}", REDIRECTION_UP),
            ObstacleType::RedirectionDown => write!(f, "{}", REDIRECTION_DOWN),
            ObstacleType::RedirectionLeft => write!(f, "{}", REDIRECTION_LEFT),
            ObstacleType::RedirectionRight => write!(f, "{}", REDIRECTION_RIGHT),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Obstacle {
    pub(crate) obstacle_type: ObstacleType,
    pub(crate) position: Point,
}

impl Obstacle {

    // Create a new obstacle from a square and a position
    // The square should be one of the following:
    // W: wall
    // R: rock
    // DU: redirection up
    // DD: redirection down
    // DL: redirection left
    // DR: redirection right
    // Return an error if the square is invalid
    pub(crate) fn new(square: String, position: Point) -> Result<Obstacle, BombermanError> {
        let obstacle_type = match square.as_str() {
            WALL => ObstacleType::Wall,
            ROCK => ObstacleType::Rock,
            REDIRECTION_UP => ObstacleType::RedirectionUp,
            REDIRECTION_DOWN => ObstacleType::RedirectionDown,
            REDIRECTION_LEFT => ObstacleType::RedirectionLeft,
            REDIRECTION_RIGHT => ObstacleType::RedirectionRight,
            _ => {
                return Err(InvalidSquare(format!(
                    "invalid obstacle {} at {}",
                    square, position
                )))
            }
        };
        Ok(Obstacle {
            obstacle_type,
            position,
        })
    }

    // Return true if the obstacle is in the position
    pub(crate) fn is_in_position(&self, position: Point) -> bool {
        self.position == position
    }

    // Return true if the explosion can pass through the obstacle
    pub(crate) fn explosion_can_pass(&self, bomb_type: BombType) -> bool {
        match bomb_type {
            BombType::Normal => {
                self.obstacle_type != ObstacleType::Wall && self.obstacle_type != ObstacleType::Rock
            }
            BombType::Penetrating => self.obstacle_type != ObstacleType::Wall,
        }
    }

    // Return the next direction to take if the obstacle is a redirection
    // Otherwise return the same direction
    pub(crate) fn next_direction(&self, direction: Direction) -> Direction {
        match self.obstacle_type {
            ObstacleType::RedirectionUp => Direction::Up,
            ObstacleType::RedirectionDown => Direction::Down,
            ObstacleType::RedirectionLeft => Direction::Left,
            ObstacleType::RedirectionRight => Direction::Right,
            _ => direction,
        }
    }
}

impl MazeDisplay for Obstacle {
    fn display(&self) -> String {
        self.obstacle_type.to_string()
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_wall() {
        let square = "W".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Ok(Obstacle {
                obstacle_type: ObstacleType::Wall,
                position: Point::new(0, 0)
            })
        );
    }

    #[test]
    fn test_new_rock() {
        let square = "R".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Ok(Obstacle {
                obstacle_type: ObstacleType::Rock,
                position: Point::new(0, 0)
            })
        );
    }

    #[test]
    fn test_new_redirection_up() {
        let square = "DU".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Ok(Obstacle {
                obstacle_type: ObstacleType::RedirectionUp,
                position: Point::new(0, 0)
            })
        );
    }

    #[test]
    fn test_new_redirection_down() {
        let square = "DD".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Ok(Obstacle {
                obstacle_type: ObstacleType::RedirectionDown,
                position: Point::new(0, 0)
            })
        );
    }

    #[test]
    fn test_new_redirection_left() {
        let square = "DL".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Ok(Obstacle {
                obstacle_type: ObstacleType::RedirectionLeft,
                position: Point::new(0, 0)
            })
        );
    }

    #[test]
    fn test_new_redirection_right() {
        let square = "DR".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Ok(Obstacle {
                obstacle_type: ObstacleType::RedirectionRight,
                position: Point::new(0, 0)
            })
        );
    }

    #[test]
    fn test_new_invalid_obstacle() {
        let square = "A".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(
            result,
            Err(InvalidSquare("invalid obstacle A at (0, 0)".to_string()))
        );
    }

    #[test]
    fn test_is_in_position_equal_position() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::Wall,
            position: Point::new(0, 0),
        };
        let position = Point::new(0, 0);
        let result = obstacle.is_in_position(position);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_in_position_different_position() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::Wall,
            position: Point::new(0, 0),
        };
        let position = Point::new(0, 1);
        let result = obstacle.is_in_position(position);
        assert_eq!(result, false);
    }

    #[test]
    fn test_next_direction_for_redirection_up() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::RedirectionUp,
            position: Point::new(0, 0),
        };
        let direction = Direction::Down;
        let result = obstacle.next_direction(direction);
        assert_eq!(result, Direction::Up);
    }

    #[test]
    fn test_next_direction_for_redirection_down() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::RedirectionDown,
            position: Point::new(0, 0),
        };
        let direction = Direction::Up;
        let result = obstacle.next_direction(direction);
        assert_eq!(result, Direction::Down);
    }

    #[test]
    fn test_next_direction_for_redirection_left() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::RedirectionLeft,
            position: Point::new(0, 0),
        };
        let direction = Direction::Right;
        let result = obstacle.next_direction(direction);
        assert_eq!(result, Direction::Left);
    }

    #[test]
    fn test_next_direction_for_redirection_right() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::RedirectionRight,
            position: Point::new(0, 0),
        };
        let direction = Direction::Left;
        let result = obstacle.next_direction(direction);
        assert_eq!(result, Direction::Right);
    }

    #[test]
    fn test_next_direction_for_not_redirection() {
        let obstacle = Obstacle {
            obstacle_type: ObstacleType::Wall,
            position: Point::new(0, 0),
        };
        let direction = Direction::Left;
        let result = obstacle.next_direction(direction);
        assert_eq!(result, Direction::Left);
    }
}

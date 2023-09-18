use crate::point::{Point, Direction};

#[derive(Debug, PartialEq)]
pub(crate) enum ObstacleType {
    Wall,
    Rock,
    RedirectionUp,
    RedirectionDown,
    RedirectionLeft,
    RedirectionRight,
}

#[derive(Debug)]
pub(crate) struct Obstacle {
    pub(crate) obstacle_type: ObstacleType,
    position: Point,
}

impl Obstacle {
    pub(crate) fn new(square: String, position: Point) -> Result<Obstacle, String> {
        let obstacle_type = match square.as_str() {
            "W" => ObstacleType::Wall,
            "R" => ObstacleType::Rock,
            "DU" => ObstacleType::RedirectionUp,
            "DD" => ObstacleType::RedirectionDown,
            "DL" => ObstacleType::RedirectionLeft,
            "DR" => ObstacleType::RedirectionRight,
            _ => return Err(format!("Invalid obstacle: {}", square)),
        };
        Ok(Obstacle {
            obstacle_type,
            position,
        })
    }

    pub(crate) fn is_in_position(&self, position: Point) -> bool {
        self.position == position
    }

    pub(crate) fn is_rock(&self) -> bool {
        self.obstacle_type == ObstacleType::Rock
    }

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

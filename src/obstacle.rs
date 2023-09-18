use crate::point::Point;

#[derive(Debug, PartialEq)]
pub(crate) enum ObstacleType {
    Wall,
    Rock,
}

#[derive(Debug)]
pub(crate) struct Obstacle {
    pub(crate) obstacle_type: ObstacleType,
    position: Point,
}

impl Obstacle {
    pub(crate) fn new(square: String, position: Point) -> Result<Obstacle, String> {
        if square.len() > 1 {
            return Err(format!("Invalid obstacle: {}", square));
        }
        let obstacle_type = match square.chars().next() {
            Some('R') => ObstacleType::Rock,
            Some('W') => ObstacleType::Wall,
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
}

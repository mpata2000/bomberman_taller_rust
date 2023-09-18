use crate::point::Point;

#[derive(Debug)]
enum ObstacleType {
    Wall,
    Rock,
}

#[derive(Debug)]
pub struct Obstacle {
    obstacle_type: ObstacleType,
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
}

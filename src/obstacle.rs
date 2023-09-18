use crate::point::Point;

enum ObstacleType {
    Wall,
    Rock,
}

pub struct Obstacle {
    obstacle_type: ObstacleType,
    position: Point,
}

impl Obstacle {
    pub(crate) fn new(square: String, position: Point) -> Result<Obstacle, String> {
        if square.len() > 1 || square[0] != 'R' && square[0] != 'W' {
            return Err(format!("Invalid obstacle: {}", square));
        }
        let obstacle_type = match square[0] {
            'R' => ObstacleType::Rock,
            'W' => ObstacleType::Wall,
            _ => return Err(format!("Invalid obstacle: {}", square)),
        };
        Ok(Obstacle {
            obstacle_type,
            position,
        })
    }
}
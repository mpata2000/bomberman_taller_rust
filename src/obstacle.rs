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

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_new_wall(){
        let square = "W".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Ok(Obstacle{obstacle_type: ObstacleType::Wall, position: Point::new(0, 0)}));
    }

    #[test]
    fn test_new_rock(){
        let square = "R".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Ok(Obstacle{obstacle_type: ObstacleType::Rock, position: Point::new(0, 0)}));
    }

    #[test]
    fn test_new_redirection_up(){
        let square = "DU".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Ok(Obstacle{obstacle_type: ObstacleType::RedirectionUp, position: Point::new(0, 0)}));
    }

    #[test]
    fn test_new_redirection_down(){
        let square = "DD".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Ok(Obstacle{obstacle_type: ObstacleType::RedirectionDown, position: Point::new(0, 0)}));
    }

    #[test]
    fn test_new_redirection_left(){
        let square = "DL".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Ok(Obstacle{obstacle_type: ObstacleType::RedirectionLeft, position: Point::new(0, 0)}));
    }

    #[test]
    fn test_new_redirection_right(){
        let square = "DR".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Ok(Obstacle{obstacle_type: ObstacleType::RedirectionRight, position: Point::new(0, 0)}));
    }

    #[test]
    fn test_new_invalid_obstacle(){
        let square = "A".to_string();
        let position = Point::new(0, 0);
        let result = Obstacle::new(square, position);
        assert_eq!(result, Err("Invalid obstacle: A".to_string()));
    }
}

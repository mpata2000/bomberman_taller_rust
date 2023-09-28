use crate::bomberman::utils::direction::Direction;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Point {
    pub(crate) fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub(crate) fn next_point(self, direction: Direction, limit: u32) -> Result<Point, String> {
        let point = match direction {
            Direction::Down if self.y < limit - 1 => Point::new(self.x, self.y + 1),
            Direction::Up if self.y > 0 => Point::new(self.x, self.y - 1),
            Direction::Left if self.x > 0 => Point::new(self.x - 1, self.y),
            Direction::Right if self.x < limit - 1 => Point::new(self.x + 1, self.y),
            _ => {
                return Err(format!(
                    "Cannot move {:?} from the {} of the board, it goes out of bounds",
                    direction, self
                ))
            }
        };
        Ok(point)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_point_up_from_start_throw_error() {
        let point = Point::new(0, 0);
        let direction = Direction::Up;
        let result = point.next_point(direction, 2);
        assert_eq!(
            result,
            Err(format!(
                "Cannot move {:?} from the {} of the board, it goes out of bounds",
                direction, point
            ))
        );
    }

    #[test]
    fn test_next_point_left_from_start_throw_error() {
        let point = Point::new(0, 0);
        let direction = Direction::Left;
        let result = point.next_point(direction, 2);
        assert_eq!(
            result,
            Err(format!(
                "Cannot move {:?} from the {} of the board, it goes out of bounds",
                direction, point
            ))
        );
    }

    #[test]
    fn test_next_point_down_from_start() {
        let point = Point::new(0, 0);
        let direction = Direction::Down;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(0, 1)));
    }

    #[test]
    fn test_next_point_right_from_start() {
        let point = Point::new(0, 0);
        let direction = Direction::Right;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(1, 0)));
    }

    #[test]
    fn test_next_point_up_from_middle() {
        let point = Point::new(0, 1);
        let direction = Direction::Up;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(0, 0)));
    }

    #[test]
    fn test_next_point_left_from_middle() {
        let point = Point::new(1, 0);
        let direction = Direction::Left;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(0, 0)));
    }

    #[test]
    fn test_next_point_down_from_middle() {
        let point = Point::new(0, 0);
        let direction = Direction::Down;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(0, 1)));
    }

    #[test]
    fn test_next_point_right_from_middle() {
        let point = Point::new(0, 0);
        let direction = Direction::Right;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(1, 0)));
    }

    #[test]
    fn test_next_point_down_from_end_throw_error() {
        let point = Point::new(0, 2);
        let direction = Direction::Down;
        let result = point.next_point(direction, 2);
        assert_eq!(
            result,
            Err(format!(
                "Cannot move {:?} from the {} of the board, it goes out of bounds",
                direction, point
            ))
        );
    }

    #[test]
    fn test_next_point_right_from_end_throw_error() {
        let point = Point::new(2, 0);
        let direction = Direction::Right;
        let result = point.next_point(direction, 2);
        assert_eq!(
            result,
            Err(format!(
                "Cannot move {:?} from the {} of the board, it goes out of bounds",
                direction, point
            ))
        );
    }

    #[test]
    fn test_next_point_up_from_end() {
        let point = Point::new(0, 2);
        let direction = Direction::Up;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(0, 1)));
    }

    #[test]
    fn test_next_point_left_from_end() {
        let point = Point::new(2, 0);
        let direction = Direction::Left;
        let result = point.next_point(direction, 2);
        assert_eq!(result, Ok(Point::new(1, 0)));
    }
}

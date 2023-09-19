use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Source: https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
    pub(crate) fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .copied()
    }
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Point {
    pub(crate) fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub(crate) fn next_point(self, direction: Direction) -> Result<Point, String> {
        let point = match direction {
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Up => {
                if self.y == 0 {
                    return Err("Cannot move down from the bottom of the board".to_string());
                }
                Point::new(self.x, self.y - 1)
            }
            Direction::Left => {
                if self.x == 0 {
                    return Err("Cannot move left from the left of the board".to_string());
                }
                Point::new(self.x - 1, self.y)
            }
            Direction::Right => Point::new(self.x + 1, self.y),
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
        let result = point.next_point(direction);
        assert_eq!(
            result,
            Err("Cannot move down from the bottom of the board".to_string())
        );
    }

    #[test]
    fn test_next_point_left_from_start_throw_error() {
        let point = Point::new(0, 0);
        let direction = Direction::Left;
        let result = point.next_point(direction);
        assert_eq!(
            result,
            Err("Cannot move left from the left of the board".to_string())
        );
    }

    #[test]
    fn test_next_point_down_from_start() {
        let point = Point::new(0, 0);
        let direction = Direction::Down;
        let result = point.next_point(direction);
        assert_eq!(result, Ok(Point::new(0, 1)));
    }

    #[test]
    fn test_next_point_right_from_start() {
        let point = Point::new(0, 0);
        let direction = Direction::Right;
        let result = point.next_point(direction);
        assert_eq!(result, Ok(Point::new(1, 0)));
    }

    #[test]
    fn test_next_point_up_from_middle() {
        let point = Point::new(0, 1);
        let direction = Direction::Up;
        let result = point.next_point(direction);
        assert_eq!(result, Ok(Point::new(0, 0)));
    }

    #[test]
    fn test_next_point_left_from_middle() {
        let point = Point::new(1, 0);
        let direction = Direction::Left;
        let result = point.next_point(direction);
        assert_eq!(result, Ok(Point::new(0, 0)));
    }
}

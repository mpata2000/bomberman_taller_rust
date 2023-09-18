#[derive(Clone, Copy)]
pub(crate) enum Direction{
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
#[derive(Debug, PartialEq,Copy,Clone)]
pub struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Point {
    pub(crate) fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub(crate) fn next_point(self, direction: Direction) -> Result<Point,String> {
        let point = match direction {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => {
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
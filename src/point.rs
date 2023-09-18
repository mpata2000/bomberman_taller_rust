pub struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Point {
    pub(crate) fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    pub(crate) fn compare(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
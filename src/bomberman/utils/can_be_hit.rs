use crate::bomberman::utils::point::Point;

pub(crate) trait CanBeHit {
    // Hit the object so it changes its state if needed
    fn hit(&mut self);
    // Return the position of the object
    fn in_position(&self, position: Point) -> bool;
}

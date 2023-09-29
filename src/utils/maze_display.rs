use crate::utils::point::Point;

pub trait MazeDisplay {
    // Return the string to display
    fn display(&self) -> String;
    // Return the position of the object
    fn get_position(&self) -> Point;
}

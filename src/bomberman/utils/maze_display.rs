use crate::bomberman::utils::point::Point;

pub(crate) trait MazeDisplay {
    // Return the string to display
    fn display(&self) -> String;
    // Return the position of the object
    fn get_position(&self) -> Point;
}

use crate::bomberman::{BombermanError, CanBeHit, MazeDisplay};
use crate::obstacle::Obstacle;
use crate::point::{Direction, Point};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum BombType {
    Normal,
    Penetrating,
}

#[derive(Debug, PartialEq)]
enum BombState {
    NotExploded,
    Exploded,
    Activated,
}
#[derive(Debug)]
pub(crate) struct Bomb {
    bomb_type: BombType,
    bomb_state: BombState,
    position: Point,
    explotion_distance: u32,
}

impl Bomb {
    pub(crate) fn new(square: String, position: Point) -> Result<Bomb, BombermanError> {
        let bomb_type = match square.chars().next() {
            Some('B') => BombType::Normal,
            Some('S') => BombType::Penetrating,
            _ => {
                return Err(BombermanError::InvalidSquare(format!(
                    "invalid bomb {} at {}. It should start with B or S",
                    square, position
                )))
            }
        };

        let explotion_distance = match square[1..].parse::<u32>() {
            Ok(bomb_distance) => bomb_distance,
            Err(_) => {
                return Err(BombermanError::InvalidSquare(format!(
                    "invalid bomb distance {} at {} it shoudld be positive number",
                    square, position
                )))
            }
        };
        Ok(Bomb {
            bomb_type,
            bomb_state: BombState::NotExploded,
            position,
            explotion_distance,
        })
    }

    pub(crate) fn is_active(&self) -> bool {
        self.bomb_state == BombState::Activated
    }

    fn bomb_can_pass(&mut self, obstacle: &Obstacle) -> bool {
        match self.bomb_type {
            BombType::Normal => obstacle.is_redirection(),
            BombType::Penetrating => obstacle.is_redirection() || obstacle.is_rock(),
        }
    }

    pub(crate) fn explode(&mut self, maze_size: u32, obstacles: &[Obstacle]) -> Vec<Point> {
        self.bomb_state = BombState::Exploded;
        let mut explosion_points = HashSet::new();
        explosion_points.insert(self.position);

        for dir in Direction::iter() {
            let mut move_dir = dir;
            let mut affected_point = self.position;
            for _ in 0..self.explotion_distance {
                affected_point = match affected_point.next_point(move_dir, maze_size) {
                    Ok(x) => x,
                    Err(_) => break,
                };

                let obstacle = obstacles
                    .iter()
                    .find(|obstacle| obstacle.is_in_position(affected_point));

                match obstacle {
                    Some(obstacle) => {
                        move_dir = obstacle.next_direction(move_dir);
                        if !self.bomb_can_pass(obstacle) {
                            break;
                        }
                        explosion_points.insert(affected_point);
                    }
                    None => {
                        explosion_points.insert(affected_point);
                    }
                }
            }
        }
        Vec::from_iter(explosion_points)
    }
}

impl CanBeHit for Bomb {
    fn hit(&mut self) {
        if self.bomb_state == BombState::NotExploded { self.bomb_state = BombState::Activated }
    }

    fn in_position(&self, position: Point) -> bool {
        self.position == position
    }
}

impl MazeDisplay for Bomb {
    fn display(&self) -> String {
        if self.bomb_state == BombState::Exploded {
            return "_".to_string();
        }
        match self.bomb_type {
            BombType::Normal => format!("B{}", self.explotion_distance),
            BombType::Penetrating => format!("S{}", self.explotion_distance),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

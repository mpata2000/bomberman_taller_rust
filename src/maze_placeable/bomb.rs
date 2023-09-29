use crate::bomberman_errors::BombermanError;
use crate::maze_placeable::bomb_state::BombState;
use crate::maze_placeable::bomb_type::BombType;
use crate::maze_placeable::obstacle::Obstacle;
use crate::utils::can_be_hit::CanBeHit;
use crate::utils::direction::Direction;
use crate::utils::maze_display::MazeDisplay;
use crate::utils::point::Point;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Bomb {
    bomb_type: BombType,
    bomb_state: BombState,
    position: Point,
    explosion_distance: u32,
}

impl Bomb {
    // Create a new bomb from a square and a position
    // The square should start with B or S and be followed by a number greater than 0
    // Return an error if the square is invalid
    pub fn new(square: String, position: Point) -> Result<Bomb, BombermanError> {
        let bomb_type = match BombType::new(&square) {
            Ok(bomb_type) => bomb_type,
            Err(_) => {
                return Err(BombermanError::InvalidSquare(format!(
                    "invalid bomb {} at {}. It should start with B or S",
                    square, position
                )))
            }
        };

        let explosion_distance = match square[1..].parse::<u32>() {
            Ok(bomb_distance) if bomb_distance > 0 => bomb_distance,
            _ => {
                return Err(BombermanError::InvalidSquare(format!(
                    "invalid bomb distance {} at {} it should be positive number greater than 0",
                    square, position
                )))
            }
        };
        Ok(Bomb {
            bomb_type,
            bomb_state: BombState::Idle,
            position,
            explosion_distance,
        })
    }

    // Return true if the bomb is active
    pub fn is_active(&self) -> bool {
        self.bomb_state == BombState::Activated
    }

    // Explode the bomb and return the list of points affected by the explosion
    pub fn explode(&mut self, maze_size: u32, obstacles: &[Obstacle]) -> Vec<Point> {
        self.bomb_state = BombState::Exploded;
        let mut explosion_points = HashSet::from([self.position]);

        for dir in Direction::iter() {
            let mut move_dir = dir;
            let mut affected_point = self.position;
            for _ in 0..self.explosion_distance {
                affected_point = match affected_point.next_point(move_dir, maze_size) {
                    Ok(x) => x,
                    Err(_) => break,
                };

                let obstacle = obstacles
                    .iter()
                    .find(|obstacle| obstacle.is_in_position(affected_point));

                match obstacle {
                    Some(obstacle) if obstacle.explosion_can_pass(self.bomb_type) => {
                        move_dir = obstacle.next_direction(move_dir);
                        explosion_points.insert(affected_point);
                    }
                    Some(_) => break,
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
    // Bomb only change state when it is idle, else it will be ignored
    fn hit(&mut self) {
        if self.bomb_state == BombState::Idle {
            self.bomb_state = BombState::Activated
        }
    }

    // Return true if the bomb is in the given position
    fn in_position(&self, position: Point) -> bool {
        self.position == position
    }
}

impl MazeDisplay for Bomb {
    // Display the bomb as B<distance> or S<distance> if it not exploded. Else it will be displayed as _
    fn display(&self) -> String {
        if self.bomb_state == BombState::Exploded {
            return "_".to_string();
        }
        match self.bomb_type {
            BombType::Normal => format!("B{}", self.explosion_distance),
            BombType::Penetrating => format!("S{}", self.explosion_distance),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::maze_placeable::obstacle_type;

    #[test]
    fn test_new_normal_bomb() {
        let bomb = Bomb::new("B3".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Ok(Bomb {
                bomb_type: BombType::Normal,
                bomb_state: BombState::Idle,
                position: Point::new(0, 0),
                explosion_distance: 3,
            })
        );
    }

    #[test]
    fn test_new_penetrating_bomb() {
        let bomb = Bomb::new("S3".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Ok(Bomb {
                bomb_type: BombType::Penetrating,
                bomb_state: BombState::Idle,
                position: Point::new(0, 0),
                explosion_distance: 3,
            })
        );
    }

    #[test]
    fn test_new_bomb_with_invalid_bomb_type() {
        let bomb = Bomb::new("A3".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Err(BombermanError::InvalidSquare(
                "invalid bomb A3 at (0, 0). It should start with B or S".to_string()
            ))
        );
    }

    #[test]
    fn test_new_bomb_with_distance_equal_to_zero_error() {
        let bomb = Bomb::new("B0".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Err(BombermanError::InvalidSquare(
                "invalid bomb distance B0 at (0, 0) it should be positive number greater than 0"
                    .to_string()
            ))
        );
    }

    #[test]
    fn test_new_bomb_with_invalid_bomb_distance() {
        let bomb = Bomb::new("Bx".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Err(BombermanError::InvalidSquare(
                "invalid bomb distance Bx at (0, 0) it should be positive number greater than 0"
                    .to_string()
            ))
        );
    }

    #[test]
    fn test_is_active() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(0, 0),
            explosion_distance: 3,
        };
        assert_eq!(bomb.is_active(), false);
        bomb.bomb_state = BombState::Activated;
        assert_eq!(bomb.is_active(), true);
    }

    #[test]
    fn test_hit_idle_bomb() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(0, 0),
            explosion_distance: 3,
        };
        bomb.hit();
        assert_eq!(bomb.bomb_state, BombState::Activated);
    }

    #[test]
    fn test_hit_activated_bomb() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explosion_distance: 3,
        };
        bomb.hit();
        assert_eq!(bomb.bomb_state, BombState::Activated);
    }

    #[test]
    fn test_hit_exploded_bomb() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Exploded,
            position: Point::new(0, 0),
            explosion_distance: 3,
        };
        bomb.hit();
        assert_eq!(bomb.bomb_state, BombState::Exploded);
    }

    #[test]
    fn test_bomb_explosion_does_not_leave_board() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 3,
        };
        let obstacles = vec![];
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }

    #[test]
    fn test_normal_bomb_can_not_penetrate_rock() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 3,
        };
        let obstacles = vec![Obstacle::new("R".to_string(), Point::new(1, 0)).unwrap()];
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }

    #[test]
    fn test_penetration_bomb_can_pass_rock() {
        let mut bomb = Bomb {
            bomb_type: BombType::Penetrating,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 3,
        };
        let obstacles = vec![];
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }

    #[test]
    fn test_normal_bomb_can_not_penetrate_wall() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 3,
        };
        let obstacles = vec![Obstacle::new("W".to_string(), Point::new(1, 0)).unwrap()];
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }

    #[test]
    fn test_penetration_bomb_can_not_penetrate_wall() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 3,
        };
        let obstacles = vec![Obstacle::new("W".to_string(), Point::new(1, 0)).unwrap()];
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }

    #[test]
    fn test_bomb_explosion_can_be_redirected() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 4,
        };
        let obstacles = vec![Obstacle::new(
            obstacle_type::REDIRECTION_LEFT.to_string(),
            Point::new(1, 0),
        )
        .unwrap()];
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
            Point::new(0, 0),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }

    #[test]
    fn test_bomb_explosion_affected_points_are_not_repeated_with_redirection() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(1, 1),
            explosion_distance: 5,
        };
        let obstacles = vec![Obstacle::new(
            obstacle_type::REDIRECTION_DOWN.to_string(),
            Point::new(1, 0),
        )
        .unwrap()];
        // The explotion up goes (1,0) -> Redirection -> (1,1) -> (1,2) -> (1,2)
        let mut explosion_points = bomb.explode(3, &obstacles);
        let mut result = vec![
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ];

        result.sort();
        explosion_points.sort();
        assert_eq!(explosion_points, result);
    }
}

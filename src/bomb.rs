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
    Idle,
    Exploded,
    Activated,
}
#[derive(Debug, PartialEq)]
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
            Ok(bomb_distance) if bomb_distance > 0 => bomb_distance,
            _ => {
                return Err(BombermanError::InvalidSquare(format!(
                    "invalid bomb distance {} at {} it shoudld be positive number greater than 0",
                    square, position
                )))
            }
        };
        Ok(Bomb {
            bomb_type,
            bomb_state: BombState::Idle,
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
        let mut explosion_points = HashSet::from([self.position]);

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
    // Bomb only change state when it is idle, else it will be ignored
    fn hit(&mut self) {
        if self.bomb_state == BombState::Idle { self.bomb_state = BombState::Activated }
    }

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
            BombType::Normal => format!("B{}", self.explotion_distance),
            BombType::Penetrating => format!("S{}", self.explotion_distance),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_new_normal_bomb() {
        let bomb = Bomb::new("B3".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Ok(Bomb {
                bomb_type: BombType::Normal,
                bomb_state: BombState::Idle,
                position: Point::new(0, 0),
                explotion_distance: 3,
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
                explotion_distance: 3,
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
                "invalid bomb distance B0 at (0, 0) it shoudld be positive number greater than 0".to_string()
            ))
        );
    }

    #[test]
    fn test_new_bomb_with_invalid_bomb_distance() {
        let bomb = Bomb::new("Bx".to_string(), Point::new(0, 0));
        assert_eq!(
            bomb,
            Err(BombermanError::InvalidSquare(
                "invalid bomb distance Bx at (0, 0) it shoudld be positive number greater than 0".to_string()
            ))
        );
    }

    #[test]
    fn test_is_active() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Idle,
            position: Point::new(0, 0),
            explotion_distance: 3,
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
            explotion_distance: 3,
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
            explotion_distance: 3,
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
            explotion_distance: 3,
        };
        bomb.hit();
        assert_eq!(bomb.bomb_state, BombState::Exploded);
    }

    #[test]
    fn test_normal_bomb_can_pass_any_redirection() {
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explotion_distance: 3,
        };
        let obstacles = vec![
            Obstacle::new("DU".to_string(), Point::new(0, 0)).unwrap(),
            Obstacle::new("DD".to_string(), Point::new(0, 0)).unwrap(),
            Obstacle::new("DL".to_string(), Point::new(0, 0)).unwrap(),
            Obstacle::new("DR".to_string(), Point::new(0, 0)).unwrap(),
        ];
        
        assert_eq!(bomb.bomb_can_pass(&obstacles[0]), true);
        assert_eq!(bomb.bomb_can_pass(&obstacles[1]), true);
        assert_eq!(bomb.bomb_can_pass(&obstacles[2]), true);
        assert_eq!(bomb.bomb_can_pass(&obstacles[3]), true);
    }

    #[test]
    fn test_penetreting_bomb_can_pass_any_redirection(){
        let mut bomb = Bomb {
            bomb_type: BombType::Penetrating,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explotion_distance: 3,
        };
        let obstacles = vec![
            Obstacle::new("DU".to_string(), Point::new(0, 0)).unwrap(),
            Obstacle::new("DD".to_string(), Point::new(0, 0)).unwrap(),
            Obstacle::new("DL".to_string(), Point::new(0, 0)).unwrap(),
            Obstacle::new("DR".to_string(), Point::new(0, 0)).unwrap(),
        ];
        
        assert_eq!(bomb.bomb_can_pass(&obstacles[0]), true);
        assert_eq!(bomb.bomb_can_pass(&obstacles[1]), true);
        assert_eq!(bomb.bomb_can_pass(&obstacles[2]), true);
        assert_eq!(bomb.bomb_can_pass(&obstacles[3]), true);
    }

    #[test]
    fn test_bomb_can_not_pass_wall(){
        let mut penetrating_bomb = Bomb {
            bomb_type: BombType::Penetrating,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explotion_distance: 3,
        };
        let mut normal_bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explotion_distance: 3,
        };

        let obstalce = Obstacle::new("W".to_string(), Point::new(0, 0)).unwrap();
        assert_eq!(penetrating_bomb.bomb_can_pass(&obstalce), false);
        assert_eq!(normal_bomb.bomb_can_pass(&obstalce), false);
    }

    #[test]
    fn test_normal_bomb_can_not_pass_rock(){
        let mut bomb = Bomb {
            bomb_type: BombType::Normal,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explotion_distance: 3,
        };

        let obstalce = Obstacle::new("R".to_string(), Point::new(0, 0)).unwrap();
        assert_eq!(bomb.bomb_can_pass(&obstalce), false);
    }

    #[test]
    fn test_penetreting_bomb_can_pass_rock(){
        let mut bomb = Bomb {
            bomb_type: BombType::Penetrating,
            bomb_state: BombState::Activated,
            position: Point::new(0, 0),
            explotion_distance: 3,
        };

        let obstalce = Obstacle::new("R".to_string(), Point::new(0, 0)).unwrap();
        assert_eq!(bomb.bomb_can_pass(&obstalce), true);
    }

    
}
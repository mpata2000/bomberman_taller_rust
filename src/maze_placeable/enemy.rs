use crate::bomberman_errors::BombermanError;
use crate::maze_placeable::enemy_state::EnemyState;
use crate::utils::can_be_hit::CanBeHit;
use crate::utils::maze_display::MazeDisplay;
use crate::utils::point::Point;

pub const ENEMY: &str = "F";

#[derive(Debug, PartialEq)]
pub struct Enemy {
    health: u32,
    position: Point,
    state: EnemyState,
}

impl Enemy {
    // Create a new enemy from a square and a position
    // The square should start with F and be followed by a number between 1 and 3 included
    // Return an error if the square is invalid
    pub fn new(square: &str, position: Point) -> Result<Enemy, BombermanError> {
        if !square.starts_with(ENEMY) {
            return Err(BombermanError::InvalidSquare(format!(
                "invalid enemy {square} at {position}"
            )));
        }

        let health = match square[1..].parse::<u32>() {
            Ok(health) if health > 0 && health < 4 => health,
            _ => {
                return Err(BombermanError::InvalidSquare(format!(
                    "invalid enemy health {square} at {position}. It should be a positive number between 1 and 3 included"
                )))
            }
        };

        Ok(Enemy {
            health,
            position,
            state: EnemyState::Idle,
        })
    }

    // If the enemy is hit, reset the state to NotHit for the next turn
    pub(crate) fn reset_state(&mut self) {
        if self.state == EnemyState::Hit {
            self.state = EnemyState::Idle;
        }
    }
}

impl CanBeHit for Enemy {
    // If the enemy is Idle, hit it and set the state to Hit
    // If the enemy is Hit or Dead, do nothing
    fn hit(&mut self) {
        match self.state {
            EnemyState::Idle => {
                self.health -= 1;
                if self.health == 0 {
                    self.state = EnemyState::Dead;
                } else {
                    self.state = EnemyState::Hit;
                }
            }
            EnemyState::Hit => (),
            EnemyState::Dead => (),
        }
    }

    // Return true if the enemy is in the given position
    fn in_position(&self, position: Point) -> bool {
        self.position == position
    }
}

impl MazeDisplay for Enemy {
    // Return the string to display
    fn display(&self) -> String {
        if self.state == EnemyState::Dead {
            return "_".to_string();
        }
        format!("F{}", self.health)
    }

    // Return the position of the object
    fn get_position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_enemy() {
        let enemy = Enemy::new("F3", Point::new(0, 0));
        assert_eq!(
            enemy,
            Ok(Enemy {
                health: 3,
                position: Point::new(0, 0),
                state: EnemyState::Idle,
            })
        );
    }

    #[test]
    fn test_new_enemy_invalid_type() {
        let enemy = Enemy::new("A3", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy A3 at (0, 0)".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_square() {
        let enemy = Enemy::new("F", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy health F at (0, 0). It should be a positive number between 1 and 3 included".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health() {
        let enemy = Enemy::new("F3A", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy health F3A at (0, 0). It should be a positive number between 1 and 3 included".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_float() {
        let enemy = Enemy::new("F3.5", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy health F3.5 at (0, 0). It should be a positive number between 1 and 3 included".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_negative() {
        let enemy = Enemy::new("F-3", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy health F-3 at (0, 0). It should be a positive number between 1 and 3 included".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_zero() {
        let enemy = Enemy::new("F0", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy health F0 at (0, 0). It should be a positive number between 1 and 3 included".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_too_high() {
        let enemy = Enemy::new("F4", Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(BombermanError::InvalidSquare(
                "invalid enemy health F4 at (0, 0). It should be a positive number between 1 and 3 included".to_string()
            ))
        );
    }

    #[test]
    fn test_reset_state_from_hit() {
        let mut enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Hit,
        };
        enemy.reset_state();
        assert_eq!(
            enemy,
            Enemy {
                health: 3,
                position: Point::new(0, 0),
                state: EnemyState::Idle,
            }
        );
    }

    #[test]
    fn test_reset_state_from_dead() {
        let mut enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Dead,
        };
        enemy.reset_state();
        assert_eq!(
            enemy,
            Enemy {
                health: 3,
                position: Point::new(0, 0),
                state: EnemyState::Dead,
            }
        );
    }

    #[test]
    fn test_reset_state_from_idle() {
        let mut enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Idle,
        };
        enemy.reset_state();
        assert_eq!(
            enemy,
            Enemy {
                health: 3,
                position: Point::new(0, 0),
                state: EnemyState::Idle,
            }
        );
    }

    #[test]
    fn test_hit_from_idle() {
        let mut enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Idle,
        };
        enemy.hit();
        assert_eq!(
            enemy,
            Enemy {
                health: 2,
                position: Point::new(0, 0),
                state: EnemyState::Hit,
            }
        );
    }

    #[test]
    fn test_hit_from_hit() {
        let mut enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Hit,
        };
        enemy.hit();
        assert_eq!(
            enemy,
            Enemy {
                health: 3,
                position: Point::new(0, 0),
                state: EnemyState::Hit,
            }
        );
    }

    #[test]
    fn test_hit_from_dead() {
        let mut enemy = Enemy {
            health: 0,
            position: Point::new(0, 0),
            state: EnemyState::Dead,
        };
        enemy.hit();
        assert_eq!(
            enemy,
            Enemy {
                health: 0,
                position: Point::new(0, 0),
                state: EnemyState::Dead,
            }
        );
    }

    #[test]
    fn test_hit_to_dead() {
        let mut enemy = Enemy {
            health: 1,
            position: Point::new(0, 0),
            state: EnemyState::Idle,
        };
        enemy.hit();
        assert_eq!(
            enemy,
            Enemy {
                health: 0,
                position: Point::new(0, 0),
                state: EnemyState::Dead,
            }
        );
    }

    #[test]
    fn test_in_position_equal_position_at_position() {
        let enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Idle,
        };
        let position = Point::new(0, 0);
        let result = enemy.in_position(position);
        assert_eq!(result, true);
    }

    #[test]
    fn test_in_position_equal_position_not_at_position() {
        let enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Idle,
        };
        let position = Point::new(0, 1);
        let result = enemy.in_position(position);
        assert_eq!(result, false);
    }

    #[test]
    fn test_display_from_idle() {
        let enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Idle,
        };
        let result = enemy.display();
        assert_eq!(result, "F3".to_string());
    }

    #[test]
    fn test_display_from_hit() {
        let enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Hit,
        };
        let result = enemy.display();
        assert_eq!(result, "F3".to_string());
    }

    #[test]
    fn test_display_from_dead() {
        let enemy = Enemy {
            health: 3,
            position: Point::new(0, 0),
            state: EnemyState::Dead,
        };
        let result = enemy.display();
        assert_eq!(result, "_".to_string());
    }
}

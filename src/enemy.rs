use crate::bomberman::BombermanError::InvalidSquare;
use crate::bomberman::{BombermanError, CanBeHit, MazeDisplay};
use crate::point::Point;

#[derive(Debug, PartialEq)]
enum EnemyState {
    Hit,
    Dead,
    Idle,
}

#[derive(Debug, PartialEq)]
pub struct Enemy {
    health: u32,
    position: Point,
    state: EnemyState,
}

impl Enemy {
    pub(crate) fn new(square: String, position: Point) -> Result<Enemy, BombermanError> {
        if !square.starts_with('F') {
            return Err(InvalidSquare(format!(
                "invalid enemy {} at {}",
                square, position
            )));
        }

        let health = match square[1..].parse::<u32>() {
            Ok(health) if health > 0 => health,
            _ => {
                return Err(InvalidSquare(format!(
                    "Invalid enemy health {} at {}. It should be a positive number",
                    square, position
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
        match self.state {
            EnemyState::Hit => self.state = EnemyState::Idle,
            _ => (),
        }
    }
}

impl CanBeHit for Enemy {
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

    fn in_position(&self, position: Point) -> bool {
        self.position == position
    }
}

impl MazeDisplay for Enemy {
    fn display(&self) -> String {
        if self.state != EnemyState::Idle {
            return "_".to_string();
        }
        format!("F{}", self.health)
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_enemy() {
        let enemy = Enemy::new("F3".to_string(), Point::new(0, 0));
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
    fn test_new_enemy_invalid_square() {
        let enemy = Enemy::new("F".to_string(), Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(InvalidSquare(
                "Invalid enemy health F at (0, 0). It should be a positive number".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health() {
        let enemy = Enemy::new("F3A".to_string(), Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(InvalidSquare(
                "Invalid enemy health F3A at (0, 0). It should be a positive number".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_float() {
        let enemy = Enemy::new("F3.5".to_string(), Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(InvalidSquare(
                "Invalid enemy health F3.5 at (0, 0). It should be a positive number".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_negative() {
        let enemy = Enemy::new("F-3".to_string(), Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(InvalidSquare(
                "Invalid enemy health F-3 at (0, 0). It should be a positive number".to_string()
            ))
        );
    }

    #[test]
    fn test_new_enemy_invalid_health_cero() {
        let enemy = Enemy::new("F0".to_string(), Point::new(0, 0));
        assert_eq!(
            enemy,
            Err(InvalidSquare(
                "Invalid enemy health F0 at (0, 0). It should be a positive number".to_string()
            ))
        );
    }
}

use crate::bomberman::{CanBeHit, MazeDisplay};
use crate::point::Point;

#[derive(Debug, PartialEq)]
enum EnemyState {
    Hit,
    Dead,
    Idle,
}

#[derive(Debug)]
pub struct Enemy {
    health: u32,
    position: Point,
    state: EnemyState,
}

impl Enemy {
    pub(crate) fn new(square: String, position: Point) -> Result<Enemy, String> {
        if square.len() < 2 {
            return Err(format!("Invalid enemy: {}", square));
        }
        match square.chars().next() {
            Some('F') => (),
            Some(_) => return Err(format!("Invalid enemy: {}", square)),
            None => return Err("Empty square string".to_string()),
        };

        let health = match square[1..].parse::<u32>() {
            Ok(health) => health,
            Err(_) => return Err(format!("Invalid enemy health: {}", square)),
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

    fn is_in_position(&self, position: Point) -> bool {
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

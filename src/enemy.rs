use crate::point::Point;

#[derive(Debug)]
enum EnemyState {
    Hit,
    Dead,
    NotHit,
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
            state: EnemyState::NotHit,
        })
    }
}

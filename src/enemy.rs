use crate::point::Point;

enum EnemyState {
    Hit,
    Dead,
    NotHit
}

pub struct Enemy {
    health: u32,
    position: Point,
    state: EnemyState,
}

impl Enemy {
    pub(crate) fn new(square: String, position: Point) -> Result<Enemy, String> {
        if square.len() < 2 || square[0] != 'F' {
            return Err(format!("Invalid enemy: {}", square));
        }
        let health = match square[1..].parse::<u32>() {
            Ok(health) => health,
            Err(_) => return Err(format!("Invalid health: {}", square)),
        };
        Ok(Enemy {
            health,
            position,
            state: EnemyState::NotHit,
        })
    }


}

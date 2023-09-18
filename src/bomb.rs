use crate::point::Point;

#[derive(Debug)]
enum BombType {
    Normal,
    Penetrating,
}

#[derive(Debug)]
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
    pub(crate) fn new(square: String, position: Point) -> Result<Bomb, String> {
        if square.len() < 2 {
            return Err(format!("Invalid bomb: {}", square));
        }

        let bomb_type = match square.chars().next() {
            Some('B') => BombType::Normal,
            Some('S') => BombType::Penetrating,
            _ => return Err(format!("Invalid bomb: {}", square)),
        };

        let explotion_distance = match square[1..].parse::<u32>() {
            Ok(bomb_distance) => bomb_distance,
            Err(_) => return Err(format!("Invalid bomb distance: {}", square)),
        };
        Ok(Bomb {
            bomb_type,
            bomb_state: BombState::NotExploded,
            position,
            explotion_distance: explotion_distance,
        })
    }
}

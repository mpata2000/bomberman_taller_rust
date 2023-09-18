use crate::point::Point;

enum BombType {
    Normal,
    Penetrating,
}

enum BombState {
    NotExploded,
    Exploded,
    Activated,
}

pub(crate) struct Bomb {
    bomb_type: BombType,
    bomb_state: BombState,
    position: Point,
    explotion_distance: u32,
}

impl Bomb {
    pub(crate) fn new(square: String, position: Point) -> Result<Bomb, String> {
        if square.len() < 2 || square[0] != 'B' && square[0] != 'S' {
            return Err(format!("Invalid bomb: {}", square));
        }
        let bomb_type = match square[0] {
            'B' => BombType::Normal,
            'S' => BombType::Penetrating,
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
pub const NORMAL_BOMB: &str = "B";
pub const PENETRATING_BOMB: &str = "S";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BombType {
    Normal,
    Penetrating,
}

impl BombType {
    pub fn new(bomb_type: &str) -> Result<Self, String> {
        match bomb_type.get(..1) {
            Some(NORMAL_BOMB) => Ok(Self::Normal),
            Some(PENETRATING_BOMB) => Ok(Self::Penetrating),
            _ => Err("Invalid bomb type".to_string()),
        }
    }
}

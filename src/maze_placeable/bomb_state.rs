#[derive(Debug, PartialEq)]
pub(super) enum BombState {
    Idle,
    Exploded,
    Activated,
}

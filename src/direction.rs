use enum_map::Enum;

/// Enum to describe the directions in which a `Piece` can move in
#[derive(Debug, Enum, Copy, Clone)]
pub enum Direction {
    /// The up direction
    Up,
    /// The right direction
    Right,
    /// The left direction
    Left,
    /// The down direction
    Down,
}

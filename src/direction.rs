use enum_map::Enum;

#[derive(Debug, Enum, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

use crate::{coordinates::Coor, direction::Direction};
use enum_map::{enum_map, EnumMap};

/// Representation of a movable piece in Klotski.
///
/// It is necessarily a rectangular piece.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Piece {
    /// Current coordinate where this `Piece` is
    pub coor: Coor,
    /// Height of this `Piece`
    pub height: usize,
    /// Width of this `Piece`
    pub width: usize,
}

impl Piece {
    /// Return all the adjacent positions to `self` organized by `Direction`.
    pub fn adjacent_spaces(&self) -> EnumMap<Direction, Vec<Coor>> {
        // `spaces` contains, for each direction, all the spaces that are adjacent to `self`
        // Note that these may include positions taken up by other pieces, but never
        // positions that are out of bounds.
        let mut spaces: EnumMap<Direction, Vec<Coor>> = enum_map! {
            Direction::Up | Direction::Down => Vec::with_capacity(self.width),
            Direction::Right | Direction::Left => Vec::with_capacity(self.height),
        };

        let upper_left = self.coor;
        let bottom_left = self.coor + Coor::new(self.height - 1, 0);
        let upper_right = self.coor + Coor::new(0, self.width - 1);

        for col in 0..self.width {
            let upper_row_coor = upper_left + Coor::new(0, col);
            if let Ok(coor) = upper_row_coor.apply_move(Direction::Up) {
                spaces[Direction::Up].push(coor);
            }

            let bottom_row_coor = bottom_left + Coor::new(0, col);
            if let Ok(coor) = bottom_row_coor.apply_move(Direction::Down) {
                spaces[Direction::Down].push(coor);
            }
        }

        for row in 0..self.height {
            let left_col_coor = upper_left + Coor::new(row, 0);
            if let Ok(coor) = left_col_coor.apply_move(Direction::Left) {
                spaces[Direction::Left].push(coor);
            }

            let right_col_coor = upper_right + Coor::new(row, 0);
            if let Ok(coor) = right_col_coor.apply_move(Direction::Right) {
                spaces[Direction::Right].push(coor);
            }
        }
        spaces
    }

    /// Return all the currently occupied coordinates by `self`.
    pub fn occupied_spaces(&self) -> Vec<Coor> {
        let mut spaces = Vec::with_capacity(self.height * self.width);

        for row in 0..self.height {
            for col in 0..self.width {
                spaces.push(self.coor + Coor::new(row, col));
            }
        }
        spaces
    }

    /// Moves `self` in the given `Direction`. Return `Err` if the move is out of bounds.
    pub fn make_move(&mut self, direction: Direction) -> Result<(), ()> {
        let new_coor = self.coor.apply_move(direction)?;
        self.coor = new_coor;
        Ok(())
    }

    /// Create a `Piece` given:
    /// - Its starting position as a `Coor`
    /// - Its height
    /// - Its width
    pub fn new(coor: Coor, height: usize, width: usize) -> Piece {
        Piece {
            coor,
            height,
            width,
        }
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let Coor { x: ax, y: ay } = self.coor;
        let Coor { x: bx, y: by } = other.coor;

        self.height
            .cmp(&other.height)
            .then(self.width.cmp(&other.width))
            .then(ax.cmp(&bx))
            .then(ay.cmp(&by))
    }
}

#[cfg(test)]
mod tests {
    use super::Piece;
    use crate::{coordinates::Coor, direction::Direction};
    use enum_map::enum_map;

    fn default_piece() -> Piece {
        Piece::new(Coor::new(0, 0), 1, 1)
    }

    #[test]
    fn check_adjacent_spaces() {
        let piece = default_piece();
        let spaces = piece.adjacent_spaces();
        assert_eq!(
            spaces,
            enum_map! {
                Direction::Up=>vec![],
                Direction::Down => vec![Coor::new(1,0)],
                Direction::Right=>vec![Coor::new(0,1)],
                Direction::Left => vec![],
            }
        );
    }

    #[test]
    fn check_boundaries_make_move() {
        let mut piece = default_piece();
        assert_eq!(piece.make_move(Direction::Up), Err(()));
    }
}

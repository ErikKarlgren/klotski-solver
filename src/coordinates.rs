use std::ops::Add;

use crate::{direction::Direction, errors::IllegalCoordinateError};

pub const ROWS: usize = 5;
pub const COLS: usize = 4;

/// `Coor` represents a coordinate of the form (row, col), where `row>=0 && col>=0'
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coor {
    pub row: usize,
    pub col: usize,
}

impl Coor {
    /// Create a new `Coor`.
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Apply a move of distance `1` to `self` in the given direction.
    ///
    /// This is how the coordinate system looks like:
    ///
    /// ```test
    ///      y=0  y=1  y=2  y=3  y=4  ...
    /// x=0
    /// x=1      
    /// x=2           (x,y)
    /// x=3
    /// x=4
    /// ...
    /// ```
    ///
    pub fn apply_move(self, direction: Direction) -> Result<Coor, IllegalCoordinateError> {
        let Coor { row, col } = self;
        let (row, col) = (row as i32, col as i32);
        let new_coor = match direction {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Left => (row, col - 1),
            Direction::Down => (row + 1, col),
        };

        let (new_row, new_col) = new_coor;

        if (0..ROWS as i32).contains(&new_row) && (0..COLS as i32).contains(&new_col) {
            Ok(Coor::new(new_coor.0 as usize, new_coor.1 as usize))
        } else {
            Err(IllegalCoordinateError)
        }
    }
}

impl Add for Coor {
    type Output = Coor;

    fn add(self, rhs: Self) -> Self::Output {
        let Coor { row, col } = self;
        let Coor {
            row: other_row,
            col: other_col,
        } = rhs;
        Coor {
            row: row + other_row,
            col: col + other_col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Coor;
    use crate::{
        coordinates::{COLS, ROWS},
        direction::Direction,
    };

    #[test]
    fn check_legal_movements() {
        let c = Coor::new(1, 1);
        assert!(c
            .apply_move(Direction::Up)
            .is_ok_and(|d| d == Coor::new(0, 1)));
        assert!(c
            .apply_move(Direction::Left)
            .is_ok_and(|d| d == Coor::new(1, 0)));
        assert!(c
            .apply_move(Direction::Right)
            .is_ok_and(|d| d == Coor::new(1, 2)));
        assert!(c
            .apply_move(Direction::Down)
            .is_ok_and(|d| d == Coor::new(2, 1)));
    }

    #[test]
    fn check_boundaries() {
        let a = Coor::new(0, 0);
        assert!(a.apply_move(Direction::Up).is_err());
        assert!(a.apply_move(Direction::Left).is_err());

        let b = Coor::new(ROWS - 1, COLS - 1);
        assert!(b.apply_move(Direction::Down).is_err());
        assert!(b.apply_move(Direction::Right).is_err());
    }
}

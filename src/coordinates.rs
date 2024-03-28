use std::ops::Add;

use crate::direction::Direction;

pub const ROWS: usize = 5;
pub const COLS: usize = 4;

/// `Coor` represents a coordinate of the form (x, y), where `x>=0 && y>=0'
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coor {
    pub x: usize,
    pub y: usize,
}

impl Coor {
    /// Create a new `Coor`.
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Apply a move of distance `1` to `self` in the given direction.
    ///
    /// This is how the coordinate system looks like:
    ///
    /// ```
    ///      y=0  y=1  y=2  y=3  y=4  ...
    /// x=0
    /// x=1      
    /// x=2           (x,y)
    /// x=3
    /// x=4
    /// ...
    /// ```
    ///
    /// TODO: Fix this so `x` refers to the horizontal axis, and `y` to the vetical one
    pub fn apply_move(self, direction: Direction) -> Result<Coor, ()> {
        let Coor { x, y } = self;
        let (x, y) = (x as i32, y as i32);
        let new_coor = match direction {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Left => (x, y - 1),
            Direction::Down => (x + 1, y),
        };

        let (new_x, new_y) = new_coor;

        if (0..ROWS as i32).contains(&new_x) && (0..COLS as i32).contains(&new_y) {
            Ok(Coor::new(new_coor.0 as usize, new_coor.1 as usize))
        } else {
            Err(())
        }
    }
}

impl Add for Coor {
    type Output = Coor;

    fn add(self, rhs: Self) -> Self::Output {
        let Coor { x, y } = self;
        let Coor { x: ox, y: oy } = rhs;
        Coor {
            x: x + ox,
            y: y + oy,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        coordinates::{COLS, ROWS},
        direction::Direction,
    };

    use super::Coor;

    #[test]
    fn check_boundaries() {
        let a = Coor::new(0, 0);
        assert_eq!(a.apply_move(Direction::Up), Err(()));
        assert_eq!(a.apply_move(Direction::Left), Err(()));

        let b = Coor::new(ROWS - 1, COLS - 1);
        assert_eq!(b.apply_move(Direction::Down), Err(()));
        assert_eq!(b.apply_move(Direction::Right), Err(()));
    }
}

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

    pub fn apply_move_to_coords(coor: Coor, direction: Direction) -> Result<Coor, ()> {
        let Coor { x, y } = coor;
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

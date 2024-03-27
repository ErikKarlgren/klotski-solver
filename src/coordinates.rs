use std::ops::Add;

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

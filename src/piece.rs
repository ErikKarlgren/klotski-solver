use crate::{coordinates::Coor, direction::Direction};
use enum_map::{enum_map, EnumMap};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Piece {
    pub coor: Coor,
    pub height: usize,
    pub width: usize,
}

impl Piece {
    pub fn adjacent_spaces(&self) -> EnumMap<Direction, Vec<Coor>> {
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

    pub fn occupied_spaces(&self) -> Vec<Coor> {
        let mut spaces = Vec::with_capacity(self.height * self.width);

        for row in 0..self.height {
            for col in 0..self.width {
                spaces.push(self.coor + Coor::new(row, col));
            }
        }
        spaces
    }

    pub fn make_move(&mut self, direction: Direction) -> Result<(), ()> {
        let new_coor = self.coor.apply_move(direction)?;
        self.coor = new_coor;
        Ok(())
    }

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

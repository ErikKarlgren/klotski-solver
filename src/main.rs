use enum_map::{enum_map, Enum, EnumMap};
use pathfinding::directed::astar;
use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    mem::size_of,
};

const ROWS: usize = 5;
const COLS: usize = 4;
const NUM_PIECES: usize = 10;
const SOLUTION: (usize, usize) = (3, 1);

type Coor = (usize, usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Piece {
    coor: Coor,
    height: usize,
    width: usize,
}

impl Piece {
    fn adjacent_spaces(&self) -> EnumMap<Direction, Vec<Coor>> {
        let mut spaces: EnumMap<Direction, Vec<Coor>> = enum_map! {
            Direction::Up => Vec::with_capacity(self.width),
            Direction::Right => Vec::with_capacity(self.height),
            Direction::Left => Vec::with_capacity(self.height),
            Direction::Down => Vec::with_capacity(self.width),
        };
        let upper_left = self.coor;
        let bottom_left = (self.coor.0 + self.height - 1, self.coor.1);
        let upper_right = (self.coor.0, self.coor.1 + self.width - 1);

        for col in 0..self.width {
            let upper_row_coor = (upper_left.0, upper_left.1 + col);
            if let Ok(coor) = apply_move_to_coords(upper_row_coor, Direction::Up) {
                spaces[Direction::Up].push(coor);
            }

            let bottom_row_coor = (bottom_left.0, bottom_left.1 + col);
            if let Ok(coor) = apply_move_to_coords(bottom_row_coor, Direction::Down) {
                spaces[Direction::Down].push(coor);
            }
        }

        for row in 0..self.height {
            let left_col_coor = (upper_left.0 + row, upper_left.1);
            if let Ok(coor) = apply_move_to_coords(left_col_coor, Direction::Left) {
                spaces[Direction::Left].push(coor);
            }

            let right_col_coor = (upper_right.0 + row, upper_right.1);
            if let Ok(coor) = apply_move_to_coords(right_col_coor, Direction::Right) {
                spaces[Direction::Right].push(coor);
            }
        }
        spaces
    }

    fn occupied_spaces(&self) -> Vec<Coor> {
        let mut spaces = Vec::with_capacity(self.height * self.width);

        for row in 0..self.height {
            for col in 0..self.width {
                spaces.push((self.coor.0 + row, self.coor.1 + col));
            }
        }
        spaces
    }

    fn move_(&mut self, direction: Direction) -> Result<(), ()> {
        let new_coor = apply_move_to_coords(self.coor, direction)?;
        self.coor = new_coor;
        Ok(())
    }

    fn new(coor: Coor, height: usize, width: usize) -> Piece {
        Piece {
            coor,
            height,
            width,
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct State {
    pieces: [Piece; NUM_PIECES],
}

impl State {
    fn new() -> State {
        State {
            pieces: [
                Piece::new((0, 1), 2, 2),
                Piece::new((0, 0), 2, 1),
                Piece::new((0, 3), 2, 1),
                Piece::new((2, 1), 1, 2),
                Piece::new((3, 0), 2, 1),
                Piece::new((3, 3), 2, 1),
                Piece::new((3, 1), 1, 1),
                Piece::new((3, 2), 1, 1),
                Piece::new((4, 1), 1, 1),
                Piece::new((4, 2), 1, 1),
            ],
        }
    }

    fn target_piece(&self) -> &Piece {
        &self.pieces[0]
    }

    fn is_solution(&self) -> bool {
        self.target_piece().coor == SOLUTION
    }

    fn to_board(&self) -> [[usize; COLS]; ROWS] {
        let mut board = [[0; COLS]; ROWS];
        for (n, piece) in self.pieces.iter().enumerate() {
            let Piece {
                coor: (x, y),
                height,
                width,
            } = piece;

            for i in 0..*height {
                for j in 0..*width {
                    if board[x + i][y + j] != 0 {
                        panic!("Cannot override value");
                    }
                    board[x + i][y + j] = n + 1;
                }
            }
        }
        board
    }

    fn available_moves(&self) -> Vec<(Piece, Direction)> {
        let mut moves: Vec<(Piece, Direction)> = vec![];
        let occupied_spaces_cache: &Vec<Vec<Coor>> =
            &self.pieces.iter().map(|p| p.occupied_spaces()).collect();

        for (n, source_piece) in self.pieces.iter().enumerate() {
            let adj_spaces = source_piece.adjacent_spaces();

            for (direction, spaces) in adj_spaces {
                if spaces.is_empty() {
                    continue;
                }

                let mut can_move = true;

                for (m, _) in self.pieces.iter().enumerate() {
                    if n == m {
                        continue;
                    } else if spaces.iter().any(|s| occupied_spaces_cache[m].contains(s)) {
                        can_move = false;
                        break;
                    }
                }

                if can_move {
                    moves.push((*source_piece, direction));
                }
            }
        }
        moves
    }

    fn next_states(&self) -> Vec<(Self, i32)> {
        let mut states = vec![];
        for (piece, dir) in self.available_moves() {
            let mut new_state = self.clone();
            new_state
                .pieces
                .iter_mut()
                .find(|p| p.coor == piece.coor)
                .unwrap()
                .move_(dir)
                .expect("Invalid move");
            states.push((new_state, 1));
        }
        states
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.to_board();
        for row in board {
            for cell in row {
                write!(f, "{:>3}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl nohash_hasher::IsEnabled for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut aux_state = self.clone();
        aux_state.pieces.sort_by(|a, b| {
            let (ax, ay) = a.coor;
            let (bx, by) = b.coor;

            a.height
                .cmp(&b.height)
                .then(a.width.cmp(&b.width))
                .then(ax.cmp(&bx))
                .then(ay.cmp(&by))
        });

        let hash = aux_state
            .pieces
            .iter()
            .flat_map(|p| {
                let (x, y) = p.coor;
                //[x & 0b111, y & 0b111]
                [x, y]
            })
            .enumerate()
            // We'll use 3 bits for each coordinate
            // That means this will only work for boards upto 8x8 squares
            .fold(0u64, |acc, (i, bits)| (bits << (i * 3)) as u64 | acc);

        state.write_u64(hash);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hasher = nohash_hasher::NoHashHasher::<State>::default();
        let mut other_hasher = nohash_hasher::NoHashHasher::<State>::default();

        self.hash(&mut self_hasher);
        other.hash(&mut other_hasher);

        self_hasher.finish() == other_hasher.finish()
    }
}

#[derive(Debug, Enum, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

// impl Direction {
//     fn opposite(&self) -> Direction {
//         match self {
//             Direction::Up => Direction::Down,
//             Direction::Right => Direction::Left,
//             Direction::Left => Direction::Right,
//             Direction::Down => Direction::Up,
//         }
//     }
// }

fn apply_move_to_coords(coor: Coor, direction: Direction) -> Result<Coor, ()> {
    let (x, y) = coor;
    let (x, y) = (x as i32, y as i32);
    let new_coor = match direction {
        Direction::Up => (x - 1, y),
        Direction::Right => (x, y + 1),
        Direction::Left => (x, y - 1),
        Direction::Down => (x + 1, y),
    };

    let (new_x, new_y) = new_coor;

    if (0..ROWS as i32).contains(&new_x) && (0..COLS as i32).contains(&new_y) {
        Ok((new_coor.0 as usize, new_coor.1 as usize))
    } else {
        Err(())
    }
}

fn main() {
    let state = State::new();
    println!("{}", state);
    println!("Size of State: {} bytes", size_of::<State>());

    let mut hasher = nohash_hasher::NoHashHasher::<State>::default();
    state.hash(&mut hasher);
    let hash = hasher.finish();
    // println!("Hash of State: {:b}", hasher.finish());
    println!("Hash of State (60 bits): {hash:060b}");

    if let Some((path, steps)) = astar::astar(
        &state,
        |p| p.next_states(),
        |s| {
            let (tx, ty) = s.target_piece().coor;
            let (sx, sy) = SOLUTION;
            (tx as i32 - sx as i32).abs() + (ty as i32 - sy as i32).abs()
        },
        |s| s.is_solution(),
    ) {
        println!("Solution found!");
        println!("Steps: {steps}");
        println!("Solution:\n{}", path.last().unwrap());
    } else {
        println!("No solution?");
    }
}

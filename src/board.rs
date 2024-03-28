use crate::{
    coordinates::{Coor, COLS, ROWS},
    direction::Direction,
    piece::Piece,
};
use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
};

pub const SOLUTION: Coor = Coor::new(3, 1);
pub const NUM_PIECES: usize = 10;

#[derive(Debug, Clone, Eq)]
pub struct Board {
    pub pieces: [Piece; NUM_PIECES],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [
                Piece::new(Coor::new(0, 1), 2, 2),
                Piece::new(Coor::new(0, 0), 2, 1),
                Piece::new(Coor::new(0, 3), 2, 1),
                Piece::new(Coor::new(2, 1), 1, 2),
                Piece::new(Coor::new(3, 0), 2, 1),
                Piece::new(Coor::new(3, 3), 2, 1),
                Piece::new(Coor::new(3, 1), 1, 1),
                Piece::new(Coor::new(3, 2), 1, 1),
                Piece::new(Coor::new(4, 1), 1, 1),
                Piece::new(Coor::new(4, 2), 1, 1),
            ],
        }
    }

    pub fn target_piece(&self) -> &Piece {
        &self.pieces[0]
    }

    pub fn is_solution(&self) -> bool {
        self.target_piece().coor == SOLUTION
    }

    fn to_board(&self) -> [[usize; COLS]; ROWS] {
        let mut board = [[0; COLS]; ROWS];
        for (n, piece) in self.pieces.iter().enumerate() {
            let Piece {
                coor: Coor { x, y },
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

    pub fn next_states(&self) -> Vec<(Self, i32)> {
        let mut states = vec![];
        for (piece, dir) in self.available_moves() {
            let mut new_state = self.clone();
            new_state
                .pieces
                .iter_mut()
                .find(|p| p.coor == piece.coor)
                .unwrap()
                .make_move(dir)
                .expect("Invalid move");
            states.push((new_state, 1));
        }
        states
    }
}

impl Display for Board {
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

impl nohash_hasher::IsEnabled for Board {}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut aux_state = self.clone();
        aux_state.pieces.sort();

        let hash = aux_state
            .pieces
            .iter()
            .flat_map(|p| {
                let Coor { x, y } = p.coor;
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

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hasher = nohash_hasher::NoHashHasher::<Board>::default();
        let mut other_hasher = nohash_hasher::NoHashHasher::<Board>::default();

        self.hash(&mut self_hasher);
        other.hash(&mut other_hasher);

        self_hasher.finish() == other_hasher.finish()
    }
}

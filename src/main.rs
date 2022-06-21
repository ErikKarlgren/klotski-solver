use enum_map::{enum_map, Enum, EnumMap};
use std::fmt::{Debug, Display};

const ROWS: usize = 5;
const COLS: usize = 4;
const NUM_PIECES: usize = 10;
const MAX_SIZE: usize = 2;

type Coor = (usize, usize);

#[derive(Debug, PartialEq)]
struct Piece {
    coor: Coor,
    height: usize,
    width: usize,
}

impl Piece {
    fn adjacent_spaces(&self) -> EnumMap<PieceMove, Vec<Coor>> {
        let mut spaces: EnumMap<PieceMove, Vec<Coor>> = enum_map! {
            PieceMove::Up => Vec::with_capacity(self.width),
            PieceMove::Right => Vec::with_capacity(self.height),
            PieceMove::Left => Vec::with_capacity(self.height),
            PieceMove::Down => Vec::with_capacity(self.width),
        };
        let upper_left = self.coor;
        let bottom_left = (self.coor.0 + self.height - 1, self.coor.1);
        let upper_right = (self.coor.0, self.coor.1 + self.width - 1);

        for col in 0..self.width {
            let upper_row_coor = (upper_left.0, upper_left.1 + col);
            if let Ok(coor) = apply_move_to_coords(upper_row_coor, PieceMove::Up) {
                spaces[PieceMove::Up].push(coor);
            }

            let bottom_row_coor = (bottom_left.0, bottom_left.1 + col);
            if let Ok(coor) = apply_move_to_coords(bottom_row_coor, PieceMove::Down) {
                spaces[PieceMove::Down].push(coor);
            }
        }

        for row in 0..self.height {
            let left_col_coor = (upper_left.0 + row, upper_left.1);
            if let Ok(coor) = apply_move_to_coords(left_col_coor, PieceMove::Left) {
                spaces[PieceMove::Left].push(coor);
            }

            let right_col_coor = (upper_right.0 + row, upper_right.1);
            if let Ok(coor) = apply_move_to_coords(right_col_coor, PieceMove::Right) {
                spaces[PieceMove::Right].push(coor);
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

    fn move_piece(&mut self, piece_move: PieceMove) {
        let (x, y) = self.coor;

        let new_coor: Coor = match piece_move {
            PieceMove::Up => (x + 1, y),
            PieceMove::Right => (x, y + 1),
            PieceMove::Left => (x, y - 1),
            PieceMove::Down => (x - 1, y),
        };

        self.coor = new_coor;
    }

    fn new(coor: Coor, height: usize, width: usize) -> Piece {
        Piece {
            coor,
            height,
            width,
        }
    }
}

#[derive(Debug)]
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

    fn target_piece_as_mut(&mut self) -> &mut Piece {
        &mut self.pieces[0]
    }

    fn to_board(&self) -> [[usize; COLS]; ROWS] {
        let mut board = [[0 as usize; COLS]; ROWS];
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

    fn available_moves(&self) -> Vec<(&Piece, PieceMove)> {
        let mut moves: Vec<(&Piece, PieceMove)> = vec![];
        let occupied_spaces_cache: &Vec<Vec<Coor>> =
            &self.pieces.iter().map(|p| p.occupied_spaces()).collect();
    
        for (n, source_piece) in self.pieces.iter().enumerate() {
            let adj_spaces = source_piece.adjacent_spaces();
    
            for (p_move, spaces) in adj_spaces {
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
                    moves.push((&source_piece, p_move));
                }
            }
        }
        moves
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

#[derive(Debug, Enum)]
enum PieceMove {
    Up,
    Right,
    Left,
    Down,
}

fn apply_move_to_coords(coor: Coor, piece_move: PieceMove) -> Result<Coor, ()> {
    let (x, y) = coor;
    let (x, y) = (x as i32, y as i32);
    let new_coor = match piece_move {
        PieceMove::Up => (x - 1, y),
        PieceMove::Right => (x, y + 1),
        PieceMove::Left => (x, y - 1),
        PieceMove::Down => (x + 1, y),
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

    for (piece, p_move) in state.available_moves() {
        println!("{piece:?}: {p_move:?}");
    }
}

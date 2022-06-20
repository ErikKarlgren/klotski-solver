use std::fmt::{Debug, Display};

const ROWS: usize = 5;
const COLS: usize = 4;
const NUM_PIECES: usize = 10;

type Coor = (usize, usize);

#[derive(Debug)]
struct Piece {
    coor: Coor,
    height: usize,
    width: usize,
}

impl Piece {
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

enum PieceMove {
    Up,
    Right,
    Left,
    Down,
}

fn main() {
    let mut state = State::new();
    println!("{}", state);

    let mut target = state.target_piece_as_mut();
    println!("{:?}", target);
    move_piece(&mut target, PieceMove::Right);
    println!("{:?}", target);
}

fn move_piece(piece: &mut Piece, piece_move: PieceMove) {
    let (x, y) = piece.coor;

    let new_coor: Coor = match piece_move {
        PieceMove::Up => (x + 1, y),
        PieceMove::Right => (x, y + 1),
        PieceMove::Left => (x, y - 1),
        PieceMove::Down => (x - 1, y),
    };

    piece.coor = new_coor;
}

mod board;
mod coordinates;
mod direction;
mod errors;
mod piece;

use board::{Board, SOLUTION};
use coordinates::Coor;
use pathfinding::directed::astar;

fn main() {
    let state = Board::new();

    if let Some((path, steps)) = solve_klotski(state) {
        println!("Solution found!");
        println!("Steps: {steps}");
        println!(
            "Solution:\n{}",
            path.last().expect("path to solution has at least 1 state")
        );
    } else {
        println!("No solution?");
    }
}

/// Solves a given Klotski board state using an A* algorithm
fn solve_klotski(state: Board) -> Option<(Vec<Board>, i32)> {
    astar::astar(
        &state,
        |p| p.next_states(),
        |s| {
            // Manhattan distance
            let Coor {
                row: target_row,
                col: target_col,
            } = s.target_piece().coor;

            let Coor {
                row: sol_row,
                col: sol_col,
            } = SOLUTION;

            (target_row as i32 - sol_row as i32).abs() + (target_col as i32 - sol_col as i32).abs()
        },
        |s| s.is_solution(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, solve_klotski};

    #[test]
    fn can_solve_default_klotski() {
        // This test may take a few seconds to complete
        let board = Board::new();
        let finished = solve_klotski(board);
        assert!(finished.is_some());
    }
}

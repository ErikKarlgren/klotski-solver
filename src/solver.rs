use crate::{
    board::{Board, SOLUTION},
    coordinates::Coor,
};
use pathfinding::directed::astar;

/// Solves a given Klotski board state using an A* algorithm
pub fn solve_klotski(state: Board) -> Option<(Vec<Board>, i32)> {
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

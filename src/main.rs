mod board;
mod coordinates;
mod direction;
mod piece;

use board::{Board, SOLUTION};
use coordinates::Coor;
use pathfinding::directed::astar;

fn main() {
    let state = Board::new();

    if let Some((path, steps)) = solve_klotski(state) {
        println!("Solution found!");
        println!("Steps: {steps}");
        println!("Solution:\n{}", path.last().unwrap());
    } else {
        println!("No solution?");
    }
}

fn solve_klotski(state: Board) -> Option<(Vec<Board>, i32)> {
    astar::astar(
        &state,
        |p| p.next_states(),
        |s| {
            // Manhattan distance
            let Coor { x: tx, y: ty } = s.target_piece().coor;
            let Coor { x: sx, y: sy } = SOLUTION;
            (tx as i32 - sx as i32).abs() + (ty as i32 - sy as i32).abs()
        },
        |s| s.is_solution(),
    )
}

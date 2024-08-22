mod board;
mod coordinates;
mod direction;
mod errors;
mod piece;
mod solver;

use board::Board;
use solver::solve_klotski;

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

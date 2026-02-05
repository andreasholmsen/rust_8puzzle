#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

// declare other modules that are in other files and must be compiled
mod board;
mod heuristics;
mod min_heap;
mod search;

// import the content of the modules
use board::*;
use heuristics::*;
use search::*;

fn main() {
    let mut board = Board::new([[1,2,3], [4,8,5], [0, 7,6]]);
    println!("{board}");

    let actions: [Direction; 4] = [Direction::Right, Direction::Up, Direction::Right, Direction::Down];


    // Works on the first 20 INSTANCES, using cargo test
}

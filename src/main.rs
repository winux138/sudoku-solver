#![warn(missing_docs)]

//! A sudoku solver using the backtrack method.

mod sudoku;

use sudoku::Sudoku;

fn main() {
    let sudoku = Sudoku::new();
    println!("Empty grid:");
    println!("{sudoku}");
}

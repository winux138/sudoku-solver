#![warn(missing_docs)]

//! A sudoku solver using the backtrack method.
//! https://en.wikipedia.org/wiki/Backtracking
//! ```text
//! procedure backtrack(P, c) is
//!     if reject(P, c) then return
//!     if accept(P, c) then output(P, c)
//!     s ← first(P, c)
//!     while s ≠ NULL do
//!         backtrack(P, s)
//!         s ← next(P, s)
//! ```

mod sudoku;

use sudoku::Sudoku;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Candidate {
    value: u8,
    row: usize,
    column: usize,
}

impl Candidate {
    const fn new() -> Self {
        Self {
            value: 1,
            row: 0,
            column: 0,
        }
    }

    /// Create the first `Candidate` (move to the next cells and restart `value` to 1) for the
    /// following cell
    const fn first(&self) -> Option<Self> {
        if self.row >= 8 && self.column >= 8 {
            None
        } else if self.column >= 8 {
            Some(Self {
                value: 1,
                row: self.row + 1,
                column: 0,
            })
        } else {
            Some(Self {
                value: 1,
                row: self.row,
                column: self.column + 1,
            })
        }
    }

    /// Create the next `Candidate` (move to the next cells and restart `value` to 1) for a given
    /// cell
    const fn next(&self) -> Option<Self> {
        if self.value >= 9 {
            None
        } else {
            Some(Self {
                value: self.value + 1,
                row: self.row,
                column: self.column,
            })
        }
    }
}

/// Return the id of the subgrid which the provided coords fall in
const fn subgrid_from_coords(row: usize, column: usize) -> usize {
    (row / 3) * 3 + (column / 3)
}

/// Return `true` if the `Candidate` does not violate any rules of sudoku
fn validate(s: &Sudoku, c: &Candidate) -> bool {
    let subgrid = subgrid_from_coords(c.row, c.column);

    !s.row(c.row).contains(&c.value)
        && !s.column(c.column).contains(&c.value)
        && !s.subgrid_flat(subgrid).contains(&c.value)
}

/// Find a solution for the given grid
/// First call must be done with the first possible `Candidate`
fn backtrack(s: &mut Sudoku, c: &Candidate) {
    if !validate(s, c) {
        return;
    }
    s.insert_at(c.value, c.row, c.column);

    let mut new_c = c.first();
    while new_c.is_some() && s.at(new_c.unwrap().row, new_c.unwrap().column) != 0 {
        new_c = new_c.unwrap().first();
    }
    if new_c.is_none() {
        println!("Solved grid:");
        println!("{s}");
    }
    while new_c.is_some() {
        backtrack(s, &new_c.unwrap());
        new_c = new_c.unwrap().next();
    }
    s.insert_at(0, c.row, c.column);
}

fn main() {
    #[rustfmt::skip]
    let mut sudoku = Sudoku::from_slice(
        [
            0, 0, 2,  0, 7, 0,  0, 5, 0, 
            0, 0, 1,  0, 3, 4,  0, 0, 0, 
            7, 0, 0,  6, 0, 0,  0, 0, 0, 
                       
            0, 0, 0,  0, 4, 0,  0, 6, 8, 
            1, 0, 0,  0, 0, 0,  0, 0, 0, 
            8, 0, 0,  0, 6, 0,  9, 0, 2, 
                       
            0, 0, 0,  5, 0, 0,  0, 7, 0, 
            0, 0, 3,  0, 2, 0,  0, 0, 1, 
            0, 4, 0,  0, 0, 6,  0, 9, 0, 
        ],
    );
    for i in 1..=9 {
        backtrack(
            &mut sudoku,
            &Candidate {
                value: i,
                row: 0,
                column: 0,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subgrid_calculation() {
        assert_eq!(subgrid_from_coords(2, 2), 0);
        assert_eq!(subgrid_from_coords(2, 5), 1);
        assert_eq!(subgrid_from_coords(2, 8), 2);
        assert_eq!(subgrid_from_coords(5, 2), 3);
        assert_eq!(subgrid_from_coords(5, 5), 4);
        assert_eq!(subgrid_from_coords(5, 8), 5);
        assert_eq!(subgrid_from_coords(8, 2), 6);
        assert_eq!(subgrid_from_coords(8, 5), 7);
        assert_eq!(subgrid_from_coords(8, 8), 8);
    }

    #[test]
    fn test_next_candidate_nominal() {
        let c = Candidate {
            value: 3,
            row: 3,
            column: 3,
        };
        assert_eq!(
            c.first(),
            Some(Candidate {
                value: 1,
                row: 3,
                column: 4,
            })
        );
    }

    #[test]
    fn test_next_candidate_last_column() {
        let c = Candidate {
            value: 3,
            row: 3,
            column: 8,
        };
        assert_eq!(
            c.first(),
            Some(Candidate {
                value: 1,
                row: 4,
                column: 0,
            })
        );
    }
}

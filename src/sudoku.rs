// TODO: Create a "value" enum that is [1-9] or Empty

pub struct Sudoku {
    grid: [u8; 81],
}

impl Sudoku {
    #[allow(unused)]
    /// Create an empty sudoku grid
    pub const fn new() -> Self {
        Self { grid: [0; 81] }
    }

    /// Create a grid from a 9x9 "flattened" array
    pub const fn from_slice(grid: [u8; 81]) -> Self {
        Self { grid }
    }

    /// Return the content of a cell at a given position
    /// row and column must be in range [0, 8]
    /// `row` and `column` must be in range [0, 8]
    pub fn at(&self, row: usize, column: usize) -> u8 {
        assert!(row < 9);
        assert!(column < 9);
        self.grid[9 * row + column]
    }

    /// Insert a new value in the grid at the given coords
    /// `value`, `row` and `column` must be in range [0, 8]
    pub fn insert_at(&mut self, value: u8, row: usize, column: usize) {
        assert!(value <= 9);
        assert!(row < 9);
        assert!(column < 9);

        self.grid[9 * row + column] = value;
    }

    /// Return the n-th row of the sudoku grid
    ///
    /// The index must be in range [0-8]
    #[allow(unused)]
    pub fn row(&self, index: usize) -> [u8; 9] {
        let mut res = [0; 9];
        for (i, item) in self.grid.into_iter().skip(index * 9).take(9).enumerate() {
            res[i] = item;
        }
        res
    }

    /// Return the n-th column of the sudoku grid
    ///
    /// The index must be in range [0-8]
    #[allow(unused)]
    pub fn column(&self, index: usize) -> [u8; 9] {
        let mut res = [0; 9];
        for (i, cell) in res.iter_mut().enumerate() {
            *cell = self.grid[9 * i + index];
        }
        res
    }

    /// Return the n-th "subgrid" of the sudoku grid
    ///
    /// The index must be in range [0-8]
    /// Subgrid indexes:
    /// 1 | 2 | 3
    /// 4 | 5 | 6
    /// 7 | 8 | 9
    #[allow(unused)]
    pub fn subgrid(&self, index: usize) -> [[u8; 3]; 3] {
        let mut res = [[0; 3]; 3];
        let subgrid_row = index / 3;
        let subgrid_column = index % 3;
        for (row_index, row) in res.iter_mut().enumerate() {
            for (column_index, column) in row.iter_mut().enumerate().take(3) {
                *column = self.grid
                    [(subgrid_row * 3 + row_index) * 9 + (subgrid_column * 3 + column_index)];
            }
        }
        res
    }

    #[allow(unused)]
    pub fn subgrid_flat(&self, index: usize) -> [u8; 9] {
        let mut res = [0; 9];
        let subgrid_row = index / 3;
        let subgrid_column = index % 3;
        for row in 0..3 {
            for column in 0..3 {
                res[row * 3 + column] =
                    self.grid[(subgrid_row * 3 + row) * 9 + (subgrid_column * 3 + column)];
            }
        }
        res
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add grid for pretty printer
        for y in 0..9 {
            for x in 0..9 {
                write!(f, "{} ", self.grid[x + y * 9])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_at_coord() {
        let sudoku = Sudoku {
            #[rustfmt::skip]
            grid: [
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 1,

            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            1, 2, 3,  4, 5, 6,  7, 8, 9,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            ],
        };

        assert_eq!(sudoku.at(5, 2), 0);
        assert_eq!(sudoku.at(2, 8), 1);
        assert_eq!(sudoku.at(6, 5), 6);
    }

    #[test]
    fn insert_at_coord() {
        let mut sudoku = Sudoku {
            #[rustfmt::skip]
            grid: [
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 1,

            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            1, 2, 3,  4, 5, 6,  7, 8, 9,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            ],
        };

        assert_eq!(sudoku.at(5, 2), 0);
        sudoku.insert_at(3, 5, 2);
        assert_eq!(sudoku.at(5, 2), 3);
    }

    #[test]
    fn get_row() {
        let sudoku = Sudoku {
            #[rustfmt::skip]
            grid: [
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            1, 2, 3,  4, 5, 6,  7, 8, 9,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            ],
        };

        assert_eq!(sudoku.row(6), [1, 2, 3, 4, 5, 6, 7, 8, 9,]);
    }

    #[test]
    fn get_column() {
        let sudoku = Sudoku {
            #[rustfmt::skip]
            grid: [
            0, 0, 0,  1, 0, 0,  0, 0, 0,
            0, 0, 0,  2, 0, 0,  0, 0, 0,
            0, 0, 0,  3, 0, 0,  0, 0, 0,

            0, 0, 0,  4, 0, 0,  0, 0, 0,
            0, 0, 0,  5, 0, 0,  0, 0, 0,
            0, 0, 0,  6, 0, 0,  0, 0, 0,

            0, 0, 0,  7, 0, 0,  0, 0, 0,
            0, 0, 0,  8, 0, 0,  0, 0, 0,
            0, 0, 0,  9, 0, 0,  0, 0, 0,
            ],
        };

        assert_eq!(sudoku.column(3), [1, 2, 3, 4, 5, 6, 7, 8, 9,]);
    }

    #[test]
    fn get_subgrid() {
        let sudoku = Sudoku {
            #[rustfmt::skip]
            grid: [
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            0, 0, 0,  1, 2, 3,  0, 0, 0,
            0, 0, 0,  4, 5, 6,  0, 0, 0,
            0, 0, 0,  7, 8, 9,  0, 0, 0,
            ],
        };

        assert_eq!(sudoku.subgrid(7), [[1, 2, 3,], [4, 5, 6,], [7, 8, 9,]]);
    }

    #[test]
    fn get_subgrid_flat() {
        let sudoku = Sudoku {
            #[rustfmt::skip]
            grid: [
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,
            0, 0, 0,  0, 0, 0,  0, 0, 0,

            0, 0, 0,  1, 2, 3,  0, 0, 0,
            0, 0, 0,  4, 5, 6,  0, 0, 0,
            0, 0, 0,  7, 8, 9,  0, 0, 0,
            ],
        };

        assert_eq!(sudoku.subgrid_flat(7), [1, 2, 3, 4, 5, 6, 7, 8, 9,]);
    }
}

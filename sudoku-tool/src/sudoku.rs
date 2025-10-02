use std::collections::BTreeSet;

use array2d::Array2D;

pub struct Sudoku {
    pub grid: Array2D<BTreeSet<u8>>,
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            grid: Array2D::filled_with(BTreeSet::new(), 9, 9),
        }
    }

    // Create a preset board from a 2D array
    pub fn from_preset(preset: [[Option<u8>; 9]; 9]) -> Self {
        let mut sudoku = Sudoku::new();

        // Set the hints
        for (row_idx, row) in preset.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                // Check for valid value
                if let Some(val) = value {
                    if val < 1 || val > 9 {
                        panic!(
                            "Invalid value {} at position ({}, {}) in preset. Values must be between 1-9.",
                            val, row_idx, col_idx
                        );
                    }

                    // Set the cell to only contain this value (solved)
                    sudoku
                        .grid
                        .set(row_idx, col_idx, BTreeSet::from([val]))
                        .unwrap();
                } else {
                    sudoku
                        .grid
                        .set(row_idx, col_idx, (1..=9).collect())
                        .unwrap();
                }
            }
        }

        sudoku
    }

    fn markup_empty_cells(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                // If this cell is solved, remove its value from peers
                if let Some(value) = self.get_solved_value(row, col) {
                    self.remove_value_from_peers(row, col);
                }
            }
        }
    }
    // Remove a solved cell's value from all cells in same row, column, and box
    fn remove_value_from_peers(&mut self, row: usize, col: usize) {
        if let Some(solved_value) = self.get_solved_value(row, col) {
            // Remove from same row
            for c in 0..9 {
                if c != col {
                    self.remove_possibility(row, c, solved_value);
                }
            }

            // Remove from same column
            for r in 0..9 {
                if r != row {
                    self.remove_possibility(r, col, solved_value);
                }
            }

            // Remove from same 3x3 box
            let box_row_start = (row / 3) * 3;
            let box_col_start = (col / 3) * 3;

            for r in box_row_start..box_row_start + 3 {
                for c in box_col_start..box_col_start + 3 {
                    if r != row || c != col {
                        self.remove_possibility(r, c, solved_value);
                    }
                }
            }
        }
    }
    // Check if a cell is solved and, if so, get the solved value of a cell
    pub fn get_solved_value(&self, row: usize, col: usize) -> Option<u8> {
        let set = self.grid.get(row, col).unwrap();
        if set.len() == 1 {
            Some(*set.iter().next().unwrap())
        } else {
            None
        }
    }
    // Remove a possibility from a cell
    pub fn remove_possibility(&mut self, row: usize, col: usize, value: u8) -> bool {
        if let Some(set) = self.grid.get_mut(row, col) {
            // Only remove from unsolved cells
            if set.len() > 1 {
                set.remove(&value)
            } else {
                false
            }
        } else {
            false
        }
    }
    // Edit a single cell. Only called before calling solver.
    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) -> Result<(), String> {
        if row >= 9 || col >= 9 {
            return Err("Invalid cell position".to_string());
        }

        if value < 1 || value > 9 {
            return Err("Value must be between 1 and 9".to_string());
        }

        self.grid
            .set(row, col, BTreeSet::from([value]))
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn is_solved(&self) -> bool {
        // Check all rows
        for row in self.grid.rows_iter() {
            if !Self::check_unit(row) {
                return false;
            }
        }

        // Check all columns
        for col in self.grid.columns_iter() {
            if !Self::check_unit(col) {
                return false;
            }
        }

        // Check all subgrids
        for i in 0..3 {
            // row index of each subgrid
            for j in 0..3 {
                // col index of each subgrid
                let subgrid_iter = (0..3).flat_map(|x| (0..3).map(move |y| (i * 3 + x, j * 3 + y)));

                // gets cell values from subgrid_iter
                let cells = subgrid_iter.map(|(r, c)| self.grid.get(r, c).unwrap());

                if !Self::check_unit(cells) {
                    return false;
                }
            }
        }

        true
    }

    fn check_unit<'a, I>(unit: I) -> bool
    where
        I: Iterator<Item = &'a BTreeSet<u8>>,
    {
        // Track numbers 1-9 (index 1-9)
        let mut seen = [false; 10];

        for cell in unit {
            // Empty, or marked cell
            if cell.len() != 1 {
                return false;
            }

            let num = cell.iter().next().unwrap();

            // Duplicate number
            if seen[*num as usize] {
                return false;
            }

            seen[*num as usize] = true;
        }

        // Check if all numbers 1-9 are present
        seen[1..=9].iter().all(|&present| present)
    }
}

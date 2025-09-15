use array2d::{Array2D, Error};

struct Sudoku {
    grid: Array2D<Option<u8>>,
}

impl Sudoku {
    fn new() -> Self {
        Sudoku {
            grid: Array2D::filled_with(None, 9, 9),
        }
    }

    fn check_complete(&self) -> bool {
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
        I: Iterator<Item = &'a Option<u8>>,
    {
        // Track numbers 1-9 (index 1-9)
        let mut seen = [false; 10];

        for cell in unit {
            match cell {
                Some(num) => {
                    if *num == 0 || *num > 9 {
                        return false; // Invalid number
                    }
                    if seen[*num as usize] {
                        return false; // Duplicate number
                    }
                    seen[*num as usize] = true;
                }
                None => return false, // Empty cell
            }
        }

        // Check if all numbers 1-9 are present
        seen[1..=9].iter().all(|&present| present)
    }

    // Create a preset board from a 2D array
    fn from_preset(preset: [[Option<u8>; 9]; 9]) -> Self {
        let mut sudoku = Sudoku::new();

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
                }

                sudoku.grid.set(row_idx, col_idx, value).unwrap();
            }
        }
        sudoku
    }

    // Edit a single cell
    fn set_cell(&mut self, row: usize, col: usize, value: Option<u8>) -> Result<(), String> {
        if row >= 9 || col >= 9 {
            return Err("Invalid cell position".to_string());
        }

        if let Some(val) = value {
            if val < 1 || val > 9 {
                return Err("Value must be between 1 and 9".to_string());
            }
        }

        self.grid.set(row, col, value).map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn main() {
    // Test with an empty board
    let empty_sudoku = Sudoku::new();
    println!("Empty board complete: {}", empty_sudoku.check_complete());
    
    // Test with a preset board
    let preset = [
        [Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(9)],
    ];
    
    let mut solved_sudoku = Sudoku::from_preset(preset);
    println!("Solved board complete: {}", solved_sudoku.check_complete());
    
    // Test editing a cell
    solved_sudoku.set_cell(0, 0, Some(5)).unwrap();
    println!("After setting cell (0,0) to 6: {}", solved_sudoku.check_complete());
    
    // Test error cases
    let result = solved_sudoku.set_cell(10, 0, Some(5));
    assert!(result.is_err());
    
    let result = solved_sudoku.set_cell(0, 0, Some(10));
    assert!(result.is_err());
    
    // This will panic with a clear error message
    let invalid_preset = [
        [Some(5), Some(3), Some(4), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(2), Some(1), Some(9), Some(5), Some(3), Some(4), Some(8)],
        [Some(1), Some(9), Some(8), Some(3), Some(4), Some(2), Some(5), Some(6), Some(7)],
        [Some(8), Some(5), Some(9), Some(7), Some(6), Some(1), Some(4), Some(2), Some(3)],
        [Some(4), Some(2), Some(6), Some(8), Some(5), Some(3), Some(7), Some(9), Some(1)],
        [Some(7), Some(1), Some(3), Some(9), Some(2), Some(4), Some(8), Some(5), Some(6)],
        [Some(9), Some(6), Some(1), Some(5), Some(3), Some(7), Some(2), Some(8), Some(4)],
        [Some(2), Some(8), Some(7), Some(4), Some(1), Some(9), Some(6), Some(3), Some(5)],
        [Some(3), Some(4), Some(5), Some(2), Some(8), Some(6), Some(1), Some(7), Some(10)], // Invalid value
    ];
    
    let sudoku = Sudoku::from_preset(invalid_preset); // This will panic
}

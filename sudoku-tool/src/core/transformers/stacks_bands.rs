use crate::core::sudoku::Sudoku;

// Given a Sudoku board and a transposition, permute the bands according to the transposition
// Pre-condition: inputs to the transposition must be 0, 1, or 2.
pub fn permute_bands(sudoku: &Sudoku, perm: (usize, usize)) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                let band = row / 3;

                let new_band = if band == perm.0 {
                    perm.1
                } else if band == perm.1 {
                    perm.0
                } else {
                    band
                };

                let new_row = (new_band * 3) + (row % 3);

                new_sudoku
                    .set_cell(new_row, col, value)
                    .expect("Failed to set cell during band permutation");
            }
        }
    }

    new_sudoku
}

// Given a Sudoku board, a band, and a transposition, permute the rows in the band according to the
// transposition.
// Pre-conditions: band and inputs to transposition must be 0, 1, or 2
pub fn permute_rows_in_band(sudoku: &Sudoku, band: usize, perm: (usize, usize)) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                let curr_band = row / 3;

                let new_row = if curr_band == band {
                    let offset = row % 3;

                    let new_offset = if offset == perm.0 {
                        perm.1
                    } else if offset == perm.1 {
                        perm.0
                    } else {
                        offset
                    };

                    (band * 3) + new_offset
                } else {
                    row
                };

                new_sudoku
                    .set_cell(new_row, col, value)
                    .expect("Failed to set cell during row permutation in band");
            }
        }
    }

    new_sudoku
}

// Given a Sudoku board and a transposition, permute the stacks according to the transposition
// Pre-condition: inputs to the transposition must be 0, 1, or 2.
pub fn permute_stacks(sudoku: &Sudoku, perm: (usize, usize)) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                let stack = col / 3;

                let new_stack = if stack == perm.0 {
                    perm.1
                } else if stack == perm.1 {
                    perm.0
                } else {
                    stack
                };

                let new_col = (new_stack * 3) + (col % 3);

                new_sudoku
                    .set_cell(row, new_col, value)
                    .expect("Failed to set cell during band permutation");
            }
        }
    }

    new_sudoku
}

// Given a Sudoku board, a stack, and a transposition, permute the cols in the stack according to the
// transposition.
// Pre-conditions: stack and inputs to transposition must be 0, 1, or 2
pub fn permute_cols_in_stack(sudoku: &Sudoku, stack: usize, perm: (usize, usize)) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                let curr_stack = col / 3;

                let new_col = if curr_stack == stack {
                    let offset = col % 3;

                    let new_offset = if offset == perm.0 {
                        perm.1
                    } else if offset == perm.1 {
                        perm.0
                    } else {
                        offset
                    };

                    (stack * 3) + new_offset
                } else {
                    col
                };

                new_sudoku
                    .set_cell(row, new_col, value)
                    .expect("Failed to set cell during row permutation in band");
            }
        }
    }

    new_sudoku
}

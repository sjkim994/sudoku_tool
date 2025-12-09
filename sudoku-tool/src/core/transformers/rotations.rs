use crate::core::sudoku::Sudoku;

/// Rotate a Sudoku puzzle 90 degrees clockwise
pub fn rotate_90(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // 90-degree rotation: (row, col) -> (col, 8 - row)
                let new_row = col;
                let new_col = 8 - row; // first col is last row --> order is flipped
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during rotation");
            }
        }
    }

    new_sudoku
}

/// Rotate a Sudoku puzzle 180 degrees
pub fn rotate_180(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // 180-degree rotation: (row, col) -> (8 - row, 8 - col)
                // both rows and columns are flipped
                let new_row = 8 - row;
                let new_col = 8 - col;
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during rotation");
            }
        }
    }

    new_sudoku
}

/// Rotate a Sudoku puzzle 270 degrees clockwise (or 90 degrees counter-clockwise)
pub fn rotate_270(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // 270-degree rotation: (row, col) -> (8 - col, row)
                let new_row = 8 - col;
                let new_col = row; // first row is last col --> order is flipped
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during rotation");
            }
        }
    }

    new_sudoku
}

/// Generate all rotational isomorphs (3 total, not including the original)
pub fn all_rotations(sudoku: &Sudoku) -> Vec<Sudoku> {
    vec![rotate_90(sudoku), rotate_180(sudoku), rotate_270(sudoku)]
}

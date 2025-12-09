use crate::core::sudoku::Sudoku;

/// Reflect a Sudoku puzzle vertically (flip across horizontal axis - top becomes bottom)
pub fn reflect_vertical(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // Vertical reflection: (row, col) -> (8 - row, col)
                let new_row = 8 - row;
                let new_col = col;
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during vertical reflection");
            }
        }
    }

    new_sudoku
}

/// Reflect a Sudoku puzzle horizontally (flip across vertical axis - left becomes right)
pub fn reflect_horizontal(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // Horizontal reflection: (row, col) -> (row, 8 - col)
                let new_row = row;
                let new_col = 8 - col;
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during horizontal reflection");
            }
        }
    }

    new_sudoku
}

/// Reflect a Sudoku puzzle across the main diagonal (top-left to bottom-right)
pub fn reflect_main_diagonal(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // Main diagonal reflection: (row, col) -> (col, row)
                let new_row = col;
                let new_col = row;
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during main diagonal reflection");
            }
        }
    }

    new_sudoku
}

/// Reflect a Sudoku puzzle across the anti-diagonal (top-right to bottom-left)
pub fn reflect_anti_diagonal(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                // Anti-diagonal reflection: (row, col) -> (8 - col, 8 - row)
                let new_row = 8 - col;
                let new_col = 8 - row;
                new_sudoku
                    .set_cell(new_row, new_col, value)
                    .expect("Failed to set cell during anti-diagonal reflection");
            }
        }
    }

    new_sudoku
}

/// Generate all reflection isomorphs (4 total)
pub fn all_reflections(sudoku: &Sudoku) -> Vec<Sudoku> {
    vec![
        reflect_vertical(sudoku),
        reflect_horizontal(sudoku),
        reflect_main_diagonal(sudoku),
        reflect_anti_diagonal(sudoku),
    ]
}

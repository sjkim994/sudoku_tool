use crate::core::sudoku::Sudoku;

// Reflect a Sudoku puzzle over the horizontal axis
pub fn h_reflect(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
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

// Reflect a Sudoku puzzle over the vertical axis
pub fn v_reflect(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
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

// Reflect a Sudoku puzzle across the main diagonal
pub fn d_reflect(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
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

// Reflect a Sudoku puzzle across the anti-diagonal
pub fn dprime_reflect(sudoku: &Sudoku) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
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

// Generate all reflection isomorphs (4 total)
pub fn all_reflections(sudoku: &Sudoku) -> Vec<Sudoku> {
    vec![
        h_reflect(sudoku),
        v_reflect(sudoku),
        d_reflect(sudoku),
        dprime_reflect(sudoku),
    ]
}

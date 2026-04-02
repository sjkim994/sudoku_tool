use crate::core::sudoku::Sudoku;

// Given a Sudoku board and a transposition, apply the transposition to the board by swapping the
// positions of each instance of the numbers
// Pre-condition: inputs to the transposition must range from 1 to 9
pub fn relabel(sudoku: &Sudoku, perm: (usize, usize)) -> Sudoku {
    let mut new_sudoku = Sudoku::new();

    let digit1 = perm.0 as u8;
    let digit2 = perm.1 as u8;

    for row in 0..9 {
        for col in 0..9 {
            if let Some(value) = sudoku.get_solved_value(row, col) {
                let new_value = if value == digit1 {
                    digit2
                } else if value == digit2 {
                    digit1
                } else {
                    value
                };

                new_sudoku
                    .set_cell(row, col, new_value)
                    .expect("Failed to set cell during relabeling");
            }
        }
    }

    new_sudoku
}


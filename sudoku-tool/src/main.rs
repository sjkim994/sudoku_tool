mod sudoku;
use sudoku::Sudoku;

mod bf_solver;

fn main() {
    // Test with an empty board
    let empty_sudoku = Sudoku::new();
    println!("Empty board complete: {}", empty_sudoku.is_solved());

    // Test with a preset board
    #[rustfmt::skip]
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
    println!("Solved board complete: {}", solved_sudoku.is_solved());

    // Test editing a cell
    solved_sudoku.set_cell(0, 0, 6).unwrap();
    println!(
        "After setting cell (0,0) to 6: {}",
        solved_sudoku.is_solved()
    );

    // Test error cases
    let result = solved_sudoku.set_cell(10, 0, 5);
    assert!(result.is_err());

    let result = solved_sudoku.set_cell(0, 0, 10);
    assert!(result.is_err());

    // This will panic with a clear error message
    #[rustfmt::skip]
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

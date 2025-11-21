use sudoku_tool::core::sudoku::Sudoku;

#[test]
fn test_empty_board() {
    let empty_sudoku = Sudoku::new();
    assert!(
        !empty_sudoku.is_solved(),
        "Empty board should not be solved"
    );
}

#[test]
fn test_solved_board() {
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

    let solved_sudoku = Sudoku::from_preset(preset);
    assert!(
        solved_sudoku.is_solved(),
        "Solved board should be marked as solved"
    );
}

#[test]
fn test_set_cell_valid() {
    let mut sudoku = Sudoku::new();
    assert!(sudoku.set_cell(0, 0, 5).is_ok());
    assert!(sudoku.set_cell(8, 8, 9).is_ok());
}

#[test]
fn test_set_cell_invalid_position() {
    let mut sudoku = Sudoku::new();
    assert!(sudoku.set_cell(10, 0, 5).is_err());
    assert!(sudoku.set_cell(0, 10, 5).is_err());
}

#[test]
fn test_set_cell_invalid_value() {
    let mut sudoku = Sudoku::new();
    assert!(sudoku.set_cell(0, 0, 0).is_err());
    assert!(sudoku.set_cell(0, 0, 10).is_err());
}

#[test]
#[should_panic(expected = "Invalid value 10")]
fn test_invalid_preset_value() {
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

    let _ = Sudoku::from_preset(invalid_preset);
}

#[test]
fn test_modify_solved_board() {
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
    assert!(solved_sudoku.is_solved());

    // Modify a cell to create a conflict
    solved_sudoku.set_cell(0, 0, 6).unwrap();
    assert!(
        !solved_sudoku.is_solved(),
        "After modification, board should not be solved"
    );
}
// Tests on from/toString
#[test]
fn test_from_string_dot_notation() {
    let puzzle = Sudoku::from_string(
        "1..5.37..6.3..8.9......98...1.......8761..........6...........7.8.9.76.47...6.312",
    )
    .expect("Valid dot notation should parse");

    // Check a few specific cells
    assert_eq!(puzzle.get_solved_value(0, 0), Some(1)); // First cell '1'
    assert_eq!(puzzle.get_solved_value(0, 1), None); // Second cell '.'
    assert_eq!(puzzle.get_solved_value(0, 2), None); // Third cell '.'
    assert_eq!(puzzle.get_solved_value(0, 3), Some(5)); // Fourth cell '5'
}

#[test]
fn test_from_string_zero_notation() {
    let puzzle = Sudoku::from_string(
        "100503700603008090000009800010000000876100000000000600000000000780907604700060312",
    )
    .expect("Valid zero notation should parse");

    // Should be identical to the dot notation version
    assert_eq!(puzzle.get_solved_value(0, 0), Some(1));
    assert_eq!(puzzle.get_solved_value(0, 1), None);
    assert_eq!(puzzle.get_solved_value(0, 2), None);
    assert_eq!(puzzle.get_solved_value(0, 3), Some(5));
}

#[test]
fn test_from_string_with_whitespace() {
    let puzzle = Sudoku::from_string("1..5  .37..6.3..  8.9......98...1  .......8761.... ......6..  .........7.8.9.76.47...6.312")
        .expect("String with whitespace should parse");

    assert_eq!(puzzle.get_solved_value(0, 0), Some(1));
    assert_eq!(puzzle.get_solved_value(0, 1), None);
}

#[test]
fn test_from_string_invalid_length() {
    let result = Sudoku::from_string("123"); // Too short
    assert!(result.is_err());

    let result = Sudoku::from_string(&"1".repeat(100)); // Too long  
    assert!(result.is_err());
}

#[test]
fn test_from_string_invalid_character() {
    let result = Sudoku::from_string(
        "1..5.37..6.3..8.9......98...1.......8761..........6...........7.8.9.76.47...6.31A",
    );
    assert!(result.is_err()); // 'A' is invalid
}

#[test]
fn test_to_string() {
    let original_string =
        "1..5.37..6.3..8.9......98...1.......8761..........6...........7.8.9.76.47...6.312";
    let puzzle = Sudoku::from_string(original_string).unwrap();
    let converted_string = puzzle.to_string();

    // The converted string should match the original (with dots for empty cells)
    assert_eq!(converted_string.len(), 81);
    // First few characters should match
    assert_eq!(&converted_string[0..4], "1..5");
}

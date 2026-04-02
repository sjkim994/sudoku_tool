use sudoku_tool::core::sudoku::Sudoku;
use sudoku_tool::core::transformers::stacks_bands::*;

#[test]
fn test_permute_bands_empty_puzzle() {
    let empty = Sudoku::new();

    // Test swapping bands 0 and 1
    let swapped = permute_bands(&empty, (0, 1));

    // All should be empty
    for row in 0..9 {
        for col in 0..9 {
            assert_eq!(swapped.get_solved_value(row, col), None);
        }
    }
}

#[test]
fn test_permute_stacks_empty_puzzle() {
    let empty = Sudoku::new();

    // Test swapping bands 0 and 1
    let swapped = permute_stacks(&empty, (0, 1));

    // All should be empty
    for row in 0..9 {
        for col in 0..9 {
            assert_eq!(swapped.get_solved_value(row, col), None);
        }
    }
}

#[test]
fn test_permute_bands_swap_0_and_1() {
    let mut sudoku = Sudoku::new();

    // Place markers in each band
    sudoku.set_cell(0, 0, 1).unwrap(); // Band 0
    sudoku.set_cell(3, 0, 2).unwrap(); // Band 1
    sudoku.set_cell(6, 0, 3).unwrap(); // Band 2

    let swapped = permute_bands(&sudoku, (0, 1));

    // Band 0 should now have the value from original band 1
    assert_eq!(swapped.get_solved_value(0, 0), Some(2));
    assert_eq!(swapped.get_solved_value(3, 0), Some(1)); // Original band 0 moved to band 1
    assert_eq!(swapped.get_solved_value(6, 0), Some(3)); // Band 2 unchanged
}

#[test]
fn test_permute_bands_swap_0_and_2() {
    let mut sudoku = Sudoku::new();

    // Place markers in each band
    sudoku.set_cell(0, 0, 1).unwrap(); // Band 0
    sudoku.set_cell(3, 0, 2).unwrap(); // Band 1
    sudoku.set_cell(6, 0, 3).unwrap(); // Band 2

    let swapped = permute_bands(&sudoku, (0, 2));

    // Band 0 should now have the value from original band 1
    assert_eq!(swapped.get_solved_value(0, 0), Some(3));
    assert_eq!(swapped.get_solved_value(3, 0), Some(2)); // Original band 0 moved to band 1
    assert_eq!(swapped.get_solved_value(6, 0), Some(1)); // Band 2 unchanged
}

#[test]
fn test_permute_bands_swap_1_and_2() {
    let mut sudoku = Sudoku::new();

    // Place markers in each band
    sudoku.set_cell(0, 0, 1).unwrap(); // Band 0
    sudoku.set_cell(3, 0, 2).unwrap(); // Band 1
    sudoku.set_cell(6, 0, 3).unwrap(); // Band 2

    let swapped = permute_bands(&sudoku, (1, 2));

    // Band 0 should now have the value from original band 1
    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(3, 0), Some(3)); // Original band 0 moved to band 1
    assert_eq!(swapped.get_solved_value(6, 0), Some(2)); // Band 2 unchanged
}

#[test]
fn test_permute_stacks_swap_0_and_1() {
    let mut sudoku = Sudoku::new();

    // Place markers in each band
    sudoku.set_cell(0, 0, 1).unwrap(); // Stack 0
    sudoku.set_cell(0, 3, 2).unwrap(); // Stack 1
    sudoku.set_cell(0, 6, 3).unwrap(); // Stack 2

    let swapped = permute_stacks(&sudoku, (0, 1));

    // Band 0 should now have the value from original band 1
    assert_eq!(swapped.get_solved_value(0, 0), Some(2));
    assert_eq!(swapped.get_solved_value(0, 3), Some(1)); // Original band 0 moved to band 1
    assert_eq!(swapped.get_solved_value(0, 6), Some(3)); // Band 2 unchanged
}

#[test]
fn test_permute_stacks_swap_0_and_2() {
    let mut sudoku = Sudoku::new();

    // Place markers in each band
    sudoku.set_cell(0, 0, 1).unwrap(); // Stack 0
    sudoku.set_cell(0, 3, 2).unwrap(); // Stack 1
    sudoku.set_cell(0, 6, 3).unwrap(); // Stack 2

    let swapped = permute_stacks(&sudoku, (0, 2));

    // Band 0 should now have the value from original band 1
    assert_eq!(swapped.get_solved_value(0, 0), Some(3));
    assert_eq!(swapped.get_solved_value(0, 3), Some(2)); // Original band 0 moved to band 1
    assert_eq!(swapped.get_solved_value(0, 6), Some(1)); // Band 2 unchanged
}

#[test]
fn test_permute_stacks_swap_1_and_2() {
    let mut sudoku = Sudoku::new();

    // Place markers in each band
    sudoku.set_cell(0, 0, 1).unwrap(); // Stack 0
    sudoku.set_cell(0, 3, 2).unwrap(); // Stack 1
    sudoku.set_cell(0, 6, 3).unwrap(); // Stack 2

    let swapped = permute_stacks(&sudoku, (1, 2));

    // Band 0 should now have the value from original band 1
    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(0, 3), Some(3)); // Original band 0 moved to band 1
    assert_eq!(swapped.get_solved_value(0, 6), Some(2)); // Band 2 unchanged
}

#[test]
fn test_permute_bands_identity() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 5, 2).unwrap();

    // Swapping a band with itself should do nothing
    let mut swapped = permute_bands(&sudoku, (0, 0));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));

    swapped = permute_bands(&sudoku, (1, 1));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));

    swapped = permute_bands(&sudoku, (2, 2));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));
}

#[test]
fn test_permute_stacks_identity() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 5, 2).unwrap();

    // Swapping a stack with itself should do nothing
    let mut swapped = permute_stacks(&sudoku, (0, 0));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));

    swapped = permute_stacks(&sudoku, (1, 1));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));

    swapped = permute_stacks(&sudoku, (2, 2));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));
}

#[test]
fn test_permute_bands_preserves_row_structure() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in band 0 rows
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(1, 0, 2).unwrap();
    sudoku.set_cell(2, 0, 3).unwrap();

    let swapped = permute_bands(&sudoku, (0, 1));

    // The pattern should move to band 1, preserving relative row order
    assert_eq!(swapped.get_solved_value(3, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 0), Some(2));
    assert_eq!(swapped.get_solved_value(5, 0), Some(3));
}

#[test]
fn test_permute_stacks_preserves_col_structure() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in stack 0 columns
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(0, 1, 2).unwrap();
    sudoku.set_cell(0, 2, 3).unwrap();

    let swapped = permute_stacks(&sudoku, (0, 2));

    // The pattern should move to stack 2, preserving relative column order
    assert_eq!(swapped.get_solved_value(0, 6), Some(1));
    assert_eq!(swapped.get_solved_value(0, 7), Some(2));
    assert_eq!(swapped.get_solved_value(0, 8), Some(3));
}

#[test]
fn test_permute_rows_in_band_swap_0_and_1() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in band 0 rows
    sudoku.set_cell(0, 0, 1).unwrap(); // Row 0 of band 0
    sudoku.set_cell(1, 0, 2).unwrap(); // Row 1 of band 0
    sudoku.set_cell(2, 0, 3).unwrap(); // Row 2 of band 0

    let swapped = permute_rows_in_band(&sudoku, 0, (0, 1));

    // Rows 0 and 1 should swap within band 0
    assert_eq!(swapped.get_solved_value(0, 0), Some(2));
    assert_eq!(swapped.get_solved_value(1, 0), Some(1));
    assert_eq!(swapped.get_solved_value(2, 0), Some(3));
}

#[test]
fn test_permute_rows_in_band_swap_0_and_2() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in band 0 rows
    sudoku.set_cell(0, 0, 1).unwrap(); // Row 0 of band 0
    sudoku.set_cell(1, 0, 2).unwrap(); // Row 1 of band 0
    sudoku.set_cell(2, 0, 3).unwrap(); // Row 2 of band 0

    let swapped = permute_rows_in_band(&sudoku, 0, (0, 2));

    // Rows 0 and 1 should swap within band 0
    assert_eq!(swapped.get_solved_value(0, 0), Some(3));
    assert_eq!(swapped.get_solved_value(1, 0), Some(2));
    assert_eq!(swapped.get_solved_value(2, 0), Some(1));
}

#[test]
fn test_permute_rows_in_band_swap_1_and_2() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in band 0 rows
    sudoku.set_cell(0, 0, 1).unwrap(); // Row 0 of band 0
    sudoku.set_cell(1, 0, 2).unwrap(); // Row 1 of band 0
    sudoku.set_cell(2, 0, 3).unwrap(); // Row 2 of band 0

    let swapped = permute_rows_in_band(&sudoku, 0, (1, 2));

    // Rows 0 and 1 should swap within band 0
    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(1, 0), Some(3));
    assert_eq!(swapped.get_solved_value(2, 0), Some(2));
}

#[test]
fn test_permute_rows_in_band_other_bands_unchanged() {
    let mut sudoku = Sudoku::new();

    // Fill patterns in all bands
    sudoku.set_cell(0, 0, 1).unwrap(); // Band 0
    sudoku.set_cell(3, 0, 2).unwrap(); // Band 1
    sudoku.set_cell(6, 0, 3).unwrap(); // Band 2

    let swapped = permute_rows_in_band(&sudoku, 1, (0, 2));

    // Only band 1 should be affected
    assert_eq!(swapped.get_solved_value(0, 0), Some(1)); // Unchanged
    assert_eq!(swapped.get_solved_value(6, 0), Some(3)); // Unchanged
    // Band 1 rows should be permuted
    assert_eq!(swapped.get_solved_value(5, 0), Some(2)); // Original row 3 (offset 0) moved to row 5 (offset 2)
}

#[test]
fn test_permute_cols_in_stack_swap_0_and_1() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in stack 0 columns
    sudoku.set_cell(0, 0, 1).unwrap(); // Col 0 of stack 0
    sudoku.set_cell(0, 1, 2).unwrap(); // Col 1 of stack 0
    sudoku.set_cell(0, 2, 3).unwrap(); // Col 2 of stack 0

    let swapped = permute_cols_in_stack(&sudoku, 0, (0, 1));

    // Columns 0 and 1 should swap within stack 0
    assert_eq!(swapped.get_solved_value(0, 0), Some(2));
    assert_eq!(swapped.get_solved_value(0, 1), Some(1));
    assert_eq!(swapped.get_solved_value(0, 2), Some(3));
}

#[test]
fn test_permute_cols_in_stack_swap_0_and_2() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in stack 0 columns
    sudoku.set_cell(0, 0, 1).unwrap(); // Col 0 of stack 0
    sudoku.set_cell(0, 1, 2).unwrap(); // Col 1 of stack 0
    sudoku.set_cell(0, 2, 3).unwrap(); // Col 2 of stack 0

    let swapped = permute_cols_in_stack(&sudoku, 0, (0, 2));

    // Columns 0 and 1 should swap within stack 0
    assert_eq!(swapped.get_solved_value(0, 0), Some(3));
    assert_eq!(swapped.get_solved_value(0, 1), Some(2));
    assert_eq!(swapped.get_solved_value(0, 2), Some(1));
}

#[test]
fn test_permute_cols_in_stack_swap_1_and_2() {
    let mut sudoku = Sudoku::new();

    // Fill a pattern in stack 0 columns
    sudoku.set_cell(0, 0, 1).unwrap(); // Col 0 of stack 0
    sudoku.set_cell(0, 1, 2).unwrap(); // Col 1 of stack 0
    sudoku.set_cell(0, 2, 3).unwrap(); // Col 2 of stack 0

    let swapped = permute_cols_in_stack(&sudoku, 0, (1, 2));

    // Columns 0 and 1 should swap within stack 0
    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(0, 1), Some(3));
    assert_eq!(swapped.get_solved_value(0, 2), Some(2));
}

#[test]
fn test_permute_cols_in_stack_other_stacks_unchanged() {
    let mut sudoku = Sudoku::new();

    // Fill patterns in all stacks
    sudoku.set_cell(0, 0, 1).unwrap(); // Stack 0
    sudoku.set_cell(0, 3, 2).unwrap(); // Stack 1
    sudoku.set_cell(0, 6, 3).unwrap(); // Stack 2

    let swapped = permute_cols_in_stack(&sudoku, 1, (0, 2));

    // Only stack 1 should be affected
    assert_eq!(swapped.get_solved_value(0, 0), Some(1)); // Unchanged
    assert_eq!(swapped.get_solved_value(0, 6), Some(3)); // Unchanged
    assert_eq!(swapped.get_solved_value(0, 5), Some(2)); // Original col 3 (offset 0) moved to col 5 (offset 2)
}

#[test]
fn test_permute_rows_in_band_identity() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 5, 2).unwrap();

    // Permuting with same offsets should do nothing
    let swapped = permute_rows_in_band(&sudoku, 0, (1, 1));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));
}

#[test]
fn test_permute_cols_in_stack_identity() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 5, 2).unwrap();

    // Permuting with same offsets should do nothing
    let swapped = permute_cols_in_stack(&sudoku, 0, (1, 1));

    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
    assert_eq!(swapped.get_solved_value(4, 5), Some(2));
}

#[test]
fn test_full_puzzle_band_permutation() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved());

    let swapped = permute_bands(&sudoku, (0, 2));

    // The result should still be solved
    assert!(
        swapped.is_solved(),
        "Band permutation should preserve solved state"
    );

    // Verify the bands were actually swapped by checking specific cells
    // Original top-left corner (0,0) is '5'
    // After swapping bands 0 and 2, original row 0 should move to row 6
    assert_eq!(swapped.get_solved_value(6, 0), Some(5));
}

#[test]
fn test_full_puzzle_row_permutation_in_band() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved());

    let swapped = permute_rows_in_band(&sudoku, 0, (0, 2));

    // The result should still be solved
    assert!(
        swapped.is_solved(),
        "Row permutation within band should preserve solved state"
    );
    assert_eq!(swapped.get_solved_value(0, 0), Some(1));
}

#[test]
fn test_full_puzzle_stack_permutation() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved());

    let swapped = permute_stacks(&sudoku, (0, 2));

    // The result should still be solved
    assert!(
        swapped.is_solved(),
        "Band permutation should preserve solved state"
    );

    // Verify the bands were actually swapped by checking specific cells
    // Original top-left corner (0,0) is '5'
    // After swapping bands 0 and 2, original row 0 should move to row 6
    assert_eq!(swapped.get_solved_value(0, 6), Some(5));
}

#[test]
fn test_full_puzzle_col_permutation_in_stack() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved());

    let swapped = permute_cols_in_stack(&sudoku, 0, (0, 2));

    // The result should still be solved
    assert!(
        swapped.is_solved(),
        "Row permutation within band should preserve solved state"
    );
    assert_eq!(swapped.get_solved_value(0, 0), Some(4));
}

use sudoku_tool::core::sudoku::Sudoku;
use sudoku_tool::core::transformers::rotations::*;

#[test]
fn test_rotate_empty_puzzle() {
    // Test with completely empty puzzle for all rotation functions
    let empty = Sudoku::new();

    let rotated_90 = rotate_90(&empty);
    let rotated_180 = rotate_180(&empty);
    let rotated_270 = rotate_270(&empty);

    // All should be empty
    assert!(!rotated_90.is_solved());
    assert!(!rotated_180.is_solved());
    assert!(!rotated_270.is_solved());

    // Check all cells are empty for each rotation
    for rotation in [&rotated_90, &rotated_180, &rotated_270] {
        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(rotation.get_solved_value(row, col), None);
            }
        }
    }

    // Also test all_rotations() function
    let all_rotations = all_rotations(&empty);
    assert_eq!(all_rotations.len(), 3);

    for rotation in all_rotations {
        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(rotation.get_solved_value(row, col), None);
            }
        }
    }
}

#[test]
fn test_rotate_90_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 5).unwrap();

    let rotated = rotate_90(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", rotated);

    // Value at (0,0) should move to (0,8)
    assert_eq!(sudoku.get_solved_value(0, 0), Some(5));
    assert_eq!(rotated.get_solved_value(0, 8), Some(5));

    // Original position should be empty
    assert_eq!(rotated.get_solved_value(0, 0), None);
}

#[test]
fn test_rotate_90_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let rotated = rotate_90(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", rotated);

    // Check rotations:
    // (0,0) -> (0,8) -> (8,8) -> (8,0) -> (0,0)
    assert_eq!(rotated.get_solved_value(0, 8), Some(1)); // top-left -> top-right
    assert_eq!(rotated.get_solved_value(8, 8), Some(2)); // top-right -> bottom-right
    assert_eq!(rotated.get_solved_value(0, 0), Some(3)); // bottom-left -> top-left
    assert_eq!(rotated.get_solved_value(8, 0), Some(4)); // bottom-right -> bottom-left
}

#[test]
fn test_rotate_180_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 5).unwrap();

    let rotated = rotate_180(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", rotated);

    // Value at (0,0) should move to (8,8)
    assert_eq!(sudoku.get_solved_value(0, 0), Some(5));
    assert_eq!(rotated.get_solved_value(8, 8), Some(5));

    // Original position should be empty
    assert_eq!(rotated.get_solved_value(0, 0), None);
}

#[test]
fn test_rotate_180_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let rotated = rotate_180(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", rotated);

    // Check rotations for 180°:
    // (0,0) -> (8,8)    (top-left -> bottom-right)
    // (0,8) -> (8,0)    (top-right -> bottom-left)
    // (8,0) -> (0,8)    (bottom-left -> top-right)
    // (8,8) -> (0,0)    (bottom-right -> top-left)
    assert_eq!(rotated.get_solved_value(8, 8), Some(1)); // top-left -> bottom-right
    assert_eq!(rotated.get_solved_value(8, 0), Some(2)); // top-right -> bottom-left
    assert_eq!(rotated.get_solved_value(0, 8), Some(3)); // bottom-left -> top-right
    assert_eq!(rotated.get_solved_value(0, 0), Some(4)); // bottom-right -> top-left
}

#[test]
fn test_rotate_270_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 5).unwrap();

    let rotated = rotate_270(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", rotated);

    // Value at (0,0) should move to (8,0) for 270° rotation
    assert_eq!(sudoku.get_solved_value(0, 0), Some(5));
    assert_eq!(rotated.get_solved_value(8, 0), Some(5));

    // Original position should be empty
    assert_eq!(rotated.get_solved_value(0, 0), None);
}

#[test]
fn test_rotate_270_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let rotated = rotate_270(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", rotated);

    // Check rotations for 270° (counter-clockwise 90°):
    // (0,0) -> (8,0)    (top-left -> bottom-left)
    // (0,8) -> (0,0)    (top-right -> top-left)
    // (8,0) -> (8,8)    (bottom-left -> bottom-right)
    // (8,8) -> (0,8)    (bottom-right -> top-right)
    assert_eq!(rotated.get_solved_value(8, 0), Some(1)); // top-left -> bottom-left
    assert_eq!(rotated.get_solved_value(0, 0), Some(2)); // top-right -> top-left
    assert_eq!(rotated.get_solved_value(8, 8), Some(3)); // bottom-left -> bottom-right
    assert_eq!(rotated.get_solved_value(0, 8), Some(4)); // bottom-right -> top-right
}

#[test]
fn test_rotate_180_identity() {
    // Rotating 180° twice should give original
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(2, 3, 7).unwrap();
    sudoku.set_cell(5, 6, 9).unwrap();

    let rotated_once = rotate_180(&sudoku);
    let rotated_twice = rotate_180(&rotated_once);

    // After two 180° rotations, values should be back in original positions
    assert_eq!(rotated_twice.get_solved_value(2, 3), Some(7));
    assert_eq!(rotated_twice.get_solved_value(5, 6), Some(9));
}

#[test]
fn test_rotate_270_is_inverse_of_90() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(1, 4, 8).unwrap();
    sudoku.set_cell(7, 2, 3).unwrap();

    // Rotate 90° then 270° should give original
    let rotated_90 = rotate_90(&sudoku);
    let rotated_back = rotate_270(&rotated_90);

    assert_eq!(rotated_back.get_solved_value(1, 4), Some(8));
    assert_eq!(rotated_back.get_solved_value(7, 2), Some(3));

    // Also rotate 270° then 90°
    let rotated_270 = rotate_270(&sudoku);
    let rotated_back2 = rotate_90(&rotated_270);

    assert_eq!(rotated_back2.get_solved_value(1, 4), Some(8));
    assert_eq!(rotated_back2.get_solved_value(7, 2), Some(3));
}

#[test]
fn test_rotate_90_full_puzzle() {
    // Create a simple pattern to verify rotation
    let mut sudoku = Sudoku::new();

    // Fill diagonal with 1-9
    for i in 0..9 {
        sudoku.set_cell(i, i, (i + 1) as u8).unwrap();
    }

    let rotated = rotate_90(&sudoku);

    // After 90° rotation, diagonal becomes anti-diagonal
    // (i,i) -> (i,8-i)
    for i in 0..9 {
        assert_eq!(rotated.get_solved_value(i, 8 - i), Some((i + 1) as u8));
    }
}

#[test]
fn test_all_rotations_length() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();

    let rotations = all_rotations(&sudoku);

    // Should have exactly 3 rotations
    assert_eq!(rotations.len(), 3);

    // Verify each is different
    let strings: Vec<String> = rotations.iter().map(|s| s.to_string()).collect();

    // All should be unique
    for i in 0..strings.len() {
        for j in i + 1..strings.len() {
            assert_ne!(
                strings[i], strings[j],
                "Rotations {} and {} should be different",
                i, j
            );
        }
    }
}

#[test]
fn test_rotation_preserves_solution_count() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved(), "Puzzle should start solved");

    let rotations = all_rotations(&sudoku);

    // All rotations of a solved puzzle should also be solved
    for (i, rotated) in rotations.iter().enumerate() {
        assert!(rotated.is_solved(), "Rotation {} should also be solved", i);
    }
}

#[test]
fn test_rotation_on_partial_puzzle() {
    // Test with a partially filled puzzle
    let partial_str =
        "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let sudoku = Sudoku::from_string(partial_str).unwrap();

    let rotations = all_rotations(&sudoku);

    // Count filled cells in original
    let mut original_filled = 0;
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.get_solved_value(row, col).is_some() {
                original_filled += 1;
            }
        }
    }

    // Each rotation should have same number of filled cells
    for (i, rotated) in rotations.iter().enumerate() {
        let mut rotated_filled = 0;
        for row in 0..9 {
            for col in 0..9 {
                if rotated.get_solved_value(row, col).is_some() {
                    rotated_filled += 1;
                }
            }
        }
        assert_eq!(
            rotated_filled, original_filled,
            "Rotation {} should have same number of filled cells",
            i
        );
    }
}

#[test]
fn test_rotation_composition() {
    // Verify that rotate_180 = rotate_90 ∘ rotate_90
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(3, 5, 9).unwrap();

    let rotated_180_direct = rotate_180(&sudoku);
    let rotated_90_twice = rotate_90(&rotate_90(&sudoku));

    assert_eq!(rotated_180_direct.to_string(), rotated_90_twice.to_string());

    // Verify that rotate_270 = rotate_90 ∘ rotate_180
    let rotated_270_direct = rotate_270(&sudoku);
    let rotated_180_then_90 = rotate_90(&rotate_180(&sudoku));

    assert_eq!(
        rotated_270_direct.to_string(),
        rotated_180_then_90.to_string()
    );
}

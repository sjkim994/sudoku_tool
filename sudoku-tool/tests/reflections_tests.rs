use sudoku_tool::core::sudoku::Sudoku;
use sudoku_tool::core::transformers::reflections::*;

#[test]
fn test_reflect_empty_puzzle() {
    // Test with completely empty puzzle for all reflection functions
    let empty = Sudoku::new();

    let reflections = [
        ("vertical", h_reflect(&empty)),
        ("horizontal", v_reflect(&empty)),
        ("main diagonal", d_reflect(&empty)),
        ("anti-diagonal", dprime_reflect(&empty)),
    ];

    for (name, reflected) in reflections.iter() {
        // All should be empty
        assert!(
            !reflected.is_solved(),
            "{} reflection should not be solved",
            name
        );

        // Check all cells are empty
        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(
                    reflected.get_solved_value(row, col),
                    None,
                    "Cell ({}, {}) should be empty after {} reflection",
                    row,
                    col,
                    name
                );
            }
        }
    }

    // Also test all_reflections() function
    let all_refs = all_reflections(&empty);
    assert_eq!(
        all_refs.len(),
        4,
        "all_reflections() should return 4 reflections"
    );
}

#[test]
fn test_reflect_vertical_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 5).unwrap();

    let reflected = h_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Value at (0,0) should move to (8,0) after vertical reflection
    assert_eq!(sudoku.get_solved_value(0, 0), Some(5));
    assert_eq!(reflected.get_solved_value(8, 0), Some(5));

    // Original position should be empty
    assert_eq!(reflected.get_solved_value(0, 0), None);
}

#[test]
fn test_reflect_vertical_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let reflected = h_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Check vertical reflection:
    // (0,0) -> (8,0)    (top-left -> bottom-left)
    // (0,8) -> (8,8)    (top-right -> bottom-right)
    // (8,0) -> (0,0)    (bottom-left -> top-left)
    // (8,8) -> (0,8)    (bottom-right -> top-right)
    assert_eq!(reflected.get_solved_value(8, 0), Some(1));
    assert_eq!(reflected.get_solved_value(8, 8), Some(2));
    assert_eq!(reflected.get_solved_value(0, 0), Some(3));
    assert_eq!(reflected.get_solved_value(0, 8), Some(4));
}

#[test]
fn test_reflect_horizontal_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 5).unwrap();

    let reflected = v_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Value at (0,0) should move to (0,8) after horizontal reflection
    assert_eq!(sudoku.get_solved_value(0, 0), Some(5));
    assert_eq!(reflected.get_solved_value(0, 8), Some(5));

    // Original position should be empty
    assert_eq!(reflected.get_solved_value(0, 0), None);
}

#[test]
fn test_reflect_horizontal_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let reflected = v_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Check horizontal reflection:
    // (0,0) -> (0,8)    (top-left -> top-right)
    // (0,8) -> (0,0)    (top-right -> top-left)
    // (8,0) -> (8,8)    (bottom-left -> bottom-right)
    // (8,8) -> (8,0)    (bottom-right -> bottom-left)
    assert_eq!(reflected.get_solved_value(0, 8), Some(1));
    assert_eq!(reflected.get_solved_value(0, 0), Some(2));
    assert_eq!(reflected.get_solved_value(8, 8), Some(3));
    assert_eq!(reflected.get_solved_value(8, 0), Some(4));
}

#[test]
fn test_reflect_main_diagonal_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 2, 5).unwrap();

    let reflected = d_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Value at (0,2) should move to (2,0) after main diagonal reflection
    assert_eq!(sudoku.get_solved_value(0, 2), Some(5));
    assert_eq!(reflected.get_solved_value(2, 0), Some(5));

    // Original position should be empty
    assert_eq!(reflected.get_solved_value(0, 2), None);
}

#[test]
fn test_reflect_main_diagonal_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let reflected = d_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Check main diagonal reflection:
    // (0,0) -> (0,0)    (top-left stays top-left)
    // (0,8) -> (8,0)    (top-right -> bottom-left)
    // (8,0) -> (0,8)    (bottom-left -> top-right)
    // (8,8) -> (8,8)    (bottom-right stays bottom-right)
    assert_eq!(reflected.get_solved_value(0, 0), Some(1));
    assert_eq!(reflected.get_solved_value(8, 0), Some(2));
    assert_eq!(reflected.get_solved_value(0, 8), Some(3));
    assert_eq!(reflected.get_solved_value(8, 8), Some(4));
}

#[test]
fn test_reflect_anti_diagonal_single_cell() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 2, 5).unwrap();

    let reflected = dprime_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Value at (0,2) should move to (6,8) after anti-diagonal reflection
    // (0,2) -> (8-2, 8-0) = (6,8)
    assert_eq!(sudoku.get_solved_value(0, 2), Some(5));
    assert_eq!(reflected.get_solved_value(6, 8), Some(5));

    // Original position should be empty
    assert_eq!(reflected.get_solved_value(0, 2), None);
}

#[test]
fn test_reflect_anti_diagonal_corner_cells() {
    let mut sudoku = Sudoku::new();
    // Place values in all four corners
    sudoku.set_cell(0, 0, 1).unwrap(); // top-left
    sudoku.set_cell(0, 8, 2).unwrap(); // top-right
    sudoku.set_cell(8, 0, 3).unwrap(); // bottom-left
    sudoku.set_cell(8, 8, 4).unwrap(); // bottom-right

    let reflected = dprime_reflect(&sudoku);

    // Allow for visual check
    println!("{}", sudoku);
    println!("{}", reflected);

    // Check anti-diagonal reflection:
    // (0,0) -> (8,8)    (top-left -> bottom-right)
    // (0,8) -> (0,8)    (top-right stays top-right)
    // (8,0) -> (8,0)    (bottom-left stays bottom-left)
    // (8,8) -> (0,0)    (bottom-right -> top-left)
    assert_eq!(reflected.get_solved_value(8, 8), Some(1));
    assert_eq!(reflected.get_solved_value(0, 8), Some(2));
    assert_eq!(reflected.get_solved_value(8, 0), Some(3));
    assert_eq!(reflected.get_solved_value(0, 0), Some(4));
}

#[test]
fn test_all_reflections_length() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();

    let reflections = all_reflections(&sudoku);

    // Should have exactly 4 reflections
    assert_eq!(reflections.len(), 4);

    // Verify each is different
    let strings: Vec<String> = reflections.iter().map(|s| s.to_string()).collect();

    // All should be unique
    for i in 0..strings.len() {
        for j in i + 1..strings.len() {
            assert_ne!(
                strings[i], strings[j],
                "Reflections {} and {} should be different",
                i, j
            );
        }
    }
}

#[test]
fn test_reflection_preserves_solution() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved(), "Puzzle should start solved");

    let reflections = all_reflections(&sudoku);

    // All reflections of a solved puzzle should also be solved
    for (i, reflected) in reflections.iter().enumerate() {
        assert!(
            reflected.is_solved(),
            "Reflection {} should also be solved",
            i
        );
    }
}

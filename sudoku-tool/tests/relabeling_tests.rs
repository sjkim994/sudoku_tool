use rand::rng;
use sudoku_tool::core::sudoku::Sudoku;
use sudoku_tool::core::transformers::relabeling::*;

#[test]
fn test_swap_digits_basic() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(0, 1, 2).unwrap();
    sudoku.set_cell(1, 0, 9).unwrap();

    let swapped = swap_digits(&sudoku, 1, 9);

    // 1 should become 9, 9 should become 1, 2 should stay 2
    assert_eq!(swapped.get_solved_value(0, 0), Some(9));
    assert_eq!(swapped.get_solved_value(0, 1), Some(2));
    assert_eq!(swapped.get_solved_value(1, 0), Some(1));
}

#[test]
fn test_swap_digits_multiple_cells() {
    let mut sudoku = Sudoku::new();
    // Create a pattern with digits 3 and 7
    sudoku.set_cell(0, 0, 3).unwrap();
    sudoku.set_cell(4, 4, 7).unwrap();
    sudoku.set_cell(8, 8, 3).unwrap();
    sudoku.set_cell(0, 8, 7).unwrap();

    let swapped = swap_digits(&sudoku, 3, 7);

    // All 3s should become 7s, all 7s should become 3s
    assert_eq!(swapped.get_solved_value(0, 0), Some(7));
    assert_eq!(swapped.get_solved_value(4, 4), Some(3));
    assert_eq!(swapped.get_solved_value(8, 8), Some(7));
    assert_eq!(swapped.get_solved_value(0, 8), Some(3));
}

#[test]
fn test_swap_digits_empty_puzzle() {
    let empty = Sudoku::new();
    let swapped = swap_digits(&empty, 1, 9);

    // Should still be empty
    for row in 0..9 {
        for col in 0..9 {
            assert_eq!(swapped.get_solved_value(row, col), None);
        }
    }
}

#[test]
#[should_panic(expected = "Cannot swap digit 5 with itself")]
fn test_swap_digits_same_digit_panic() {
    let sudoku = Sudoku::new();
    swap_digits(&sudoku, 5, 5); // Should panic
}

#[test]
#[should_panic(expected = "Digits must be between 1 and 9")]
fn test_swap_digits_invalid_digit_low() {
    let sudoku = Sudoku::new();
    swap_digits(&sudoku, 0, 5); // Should panic
}

#[test]
#[should_panic(expected = "Digits must be between 1 and 9")]
fn test_swap_digits_invalid_digit_high() {
    let sudoku = Sudoku::new();
    swap_digits(&sudoku, 1, 10); // Should panic
}

#[test]
fn test_relabel_identity() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 4, 5).unwrap();
    sudoku.set_cell(8, 8, 9).unwrap();

    // Identity permutation
    let identity = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let relabeled = relabel(&sudoku, &identity).unwrap();

    // Should be identical
    assert_eq!(relabeled.get_solved_value(0, 0), Some(1));
    assert_eq!(relabeled.get_solved_value(4, 4), Some(5));
    assert_eq!(relabeled.get_solved_value(8, 8), Some(9));
}

#[test]
fn test_relabel_reverse() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 4, 5).unwrap();
    sudoku.set_cell(8, 8, 9).unwrap();

    // Reverse permutation
    let reverse = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let relabeled = relabel(&sudoku, &reverse).unwrap();

    // 1→9, 5→5, 9→1
    assert_eq!(relabeled.get_solved_value(0, 0), Some(9));
    assert_eq!(relabeled.get_solved_value(4, 4), Some(5)); // 5 maps to itself in reverse
    assert_eq!(relabeled.get_solved_value(8, 8), Some(1));
}

#[test]
#[should_panic(expected = "Invalid digit 10 in permutation")]
fn test_relabel_invalid_digit() {
    let sudoku = Sudoku::new();
    let invalid_perm = [10, 2, 3, 4, 5, 6, 7, 8, 9]; // 10 is invalid
    relabel(&sudoku, &invalid_perm).unwrap();
}

#[test]
#[should_panic(expected = "Duplicate digit 1 in permutation")]
fn test_relabel_duplicate_digit() {
    let sudoku = Sudoku::new();
    let duplicate_perm = [1, 1, 3, 4, 5, 6, 7, 8, 9]; // 1 appears twice
    relabel(&sudoku, &duplicate_perm).unwrap();
}

#[test]
fn test_random_permutation_valid() {
    let mut rng = rng();
    let perm = random_permutation(&mut rng);

    // Should contain all digits 1-9
    let mut digits: Vec<u8> = perm.to_vec();
    digits.sort();
    assert_eq!(digits, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_random_relabel() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 4, 5).unwrap();
    sudoku.set_cell(8, 8, 9).unwrap();

    let mut rng = rng();
    let relabeled = random_relabel(&sudoku, &mut rng);

    // Should still have 3 filled cells (just with different digits)
    let mut filled_count = 0;
    for row in 0..9 {
        for col in 0..9 {
            if relabeled.get_solved_value(row, col).is_some() {
                filled_count += 1;
            }
        }
    }
    assert_eq!(filled_count, 3);
}

#[test]
fn test_all_pairwise_swaps_count() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();

    let swaps = all_pairwise_swaps(&sudoku);

    // C(9,2) = 36 unique swaps
    assert_eq!(swaps.len(), 36);

    // All should have exactly 1 filled cell
    for swap in &swaps {
        let mut filled_count = 0;
        for row in 0..9 {
            for col in 0..9 {
                if swap.get_solved_value(row, col).is_some() {
                    filled_count += 1;
                }
            }
        }
        assert_eq!(filled_count, 1);
    }
}

#[test]
fn test_all_pairwise_swaps_uniqueness() {
    let mut sudoku = Sudoku::new();

    // Put each digit in a different cell
    for i in 0..9 {
        sudoku.set_cell(0, i, (i + 1) as u8).unwrap();
    }

    let swaps = all_pairwise_swaps(&sudoku);

    // Should have 36 total swaps
    assert_eq!(swaps.len(), 36);

    // All swaps should be unique
    let strings: Vec<String> = swaps.iter().map(|s| s.to_string_rep()).collect();
    for i in 0..strings.len() {
        for j in (i + 1)..strings.len() {
            assert_ne!(
                strings[i], strings[j],
                "Swaps {} and {} should be different",
                i, j
            );
        }
    }
}

#[test]
fn test_sample_pairwise_swaps() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(0, 1, 2).unwrap();

    let mut rng = rng();
    let sample_size = 5;
    let swaps = sample_pairwise_swaps(&sudoku, &mut rng, sample_size);

    assert_eq!(swaps.len(), sample_size);

    // All should have 2 filled cells
    for swap in swaps {
        let mut filled_count = 0;
        for row in 0..9 {
            for col in 0..9 {
                if swap.get_solved_value(row, col).is_some() {
                    filled_count += 1;
                }
            }
        }
        assert_eq!(filled_count, 2);
    }
}

#[test]
fn test_sample_pairwise_swaps_all_pairs() {
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();

    let mut rng = rng();
    let sample_size = 100; // Larger than 36, should only return 36
    let swaps = sample_pairwise_swaps(&sudoku, &mut rng, sample_size);

    // Should be capped at 36 (all possible swaps)
    assert_eq!(swaps.len(), 36);
}

#[test]
fn test_relabel_preserves_solution() {
    // Test with a solved puzzle
    let solved_str =
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179";
    let sudoku = Sudoku::from_string(solved_str).unwrap();

    assert!(sudoku.is_solved(), "Puzzle should start solved");

    // Test swap
    let swapped = swap_digits(&sudoku, 1, 9);
    assert!(swapped.is_solved(), "Swapped puzzle should also be solved");

    // Test random relabel
    let mut rng = rng();
    let random = random_relabel(&sudoku, &mut rng);
    assert!(random.is_solved(), "Random relabel should also be solved");
}

#[test]
fn test_relabel_inverse_operations() {
    // Relabeling twice with inverse permutations should give original
    let mut sudoku = Sudoku::new();
    sudoku.set_cell(0, 0, 1).unwrap();
    sudoku.set_cell(4, 4, 5).unwrap();

    // Test swap inverse
    let swapped = swap_digits(&sudoku, 1, 9);
    let swapped_back = swap_digits(&swapped, 1, 9);
    assert_eq!(swapped_back.to_string_rep(), sudoku.to_string_rep());
}

#[test]
fn test_relabel_with_complex_puzzle() {
    // Test with a more complex, partially filled puzzle
    let partial_str =
        "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    let sudoku = Sudoku::from_string(partial_str).unwrap();

    let mut rng = rng();

    // Count original filled cells
    let mut original_filled = 0;
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.get_solved_value(row, col).is_some() {
                original_filled += 1;
            }
        }
    }

    // Test random relabel preserves filled count
    let random = random_relabel(&sudoku, &mut rng);
    let mut random_filled = 0;
    for row in 0..9 {
        for col in 0..9 {
            if random.get_solved_value(row, col).is_some() {
                random_filled += 1;
            }
        }
    }
    assert_eq!(random_filled, original_filled);

    // Test swap preserves filled count
    let swapped = swap_digits(&sudoku, 1, 9);
    let mut swapped_filled = 0;
    for row in 0..9 {
        for col in 0..9 {
            if swapped.get_solved_value(row, col).is_some() {
                swapped_filled += 1;
            }
        }
    }
    assert_eq!(swapped_filled, original_filled);
}

use sudoku_tool::core::solvers::bf_solver::*;
use sudoku_tool::core::sudoku::Sudoku;

#[test]
fn test_solve_empty_puzzle() {
    let puzzle = Sudoku::new();
    let (solution, stats) = find_one_solution(&puzzle);

    assert!(solution.is_some(), "Empty puzzle should have a solution");
    assert!(stats.solutions_found == 1);
    assert!(stats.nodes_explored > 0);

    println!("{}", puzzle)
}

#[test]
fn test_order_coverage() {
    let puzzle = Sudoku::new(); // Empty puzzle
    let row_order: [usize; 9] = [0, 1, 2, 4, 3, 5, 6, 7, 8];
    let col_order: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    let (solution, stats) = find_one_solution_ord(&puzzle, Some(row_order), Some(col_order));
    
    assert!(solution.is_some(), "Empty puzzle should always have a solution regardless of order");
    println!("Nodes explored: {}", stats.nodes_explored);
    
    // An empty puzzle should explore roughly the same number of nodes regardless of order
    // If this fails, there's definitely a bug in the ordering logic
}

#[test]
fn test_solve_already_solved_puzzle() {
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

    let puzzle = Sudoku::from_preset(preset);

    let (solution, stats) = find_one_solution(&puzzle);
    assert!(
        solution.is_some(),
        "Already solved puzzle should return a solution"
    );
    // Should find solution very quickly (minimal nodes explored)

    // You could also add this assertion to verify it was indeed fast:
    assert!(
        stats.nodes_explored <= 81,
        "Solved puzzle should require minimal exploration"
    );
}

#[test]
fn test_shultz_301() {
    #[rustfmt::skip]
    let preset = [
        [None,    Some(3), Some(9), Some(5), None,     None,     None,     None,     None    ],
        [None,    None,    None,    Some(8), None,     None,     None,     Some(7),  None    ],
        [None,    None,    None,    None,    Some(1),  None,     Some(9),  None,     Some(4) ],
        [Some(1), None,    None,    Some(4), None,     None,     None,     None,     Some(3) ],
        [None,    None,    None,    None,    None,     None,     None,     None,     None    ],
        [None,    None,    Some(7), None,    None,     None,     Some(8),  Some(6),  None    ],
        [None,    None,    Some(6), Some(7), None,     Some(8),  Some(2),  None,     None    ],
        [None,    Some(1), None,    None,    Some(9),  None,     None,     None,     Some(5) ],
        [None,    None,    None,    None,    None,     Some(1),  None,     None,     Some(8) ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    let (solution, stats) = find_one_solution(&puzzle);
    assert!(solution.is_some(), "Puzzle should have a solution");
    assert!(stats.solutions_found == 1);

    // Optional: verify the solution is actually valid
    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
        println!("{}", solved_puzzle);

        stats.print_analysis();
    }
}

#[test]
fn test_shultz_301_rand() {
    #[rustfmt::skip]
    let preset = [
        [None,    Some(3), Some(9), Some(5), None,     None,     None,     None,     None    ],
        [None,    None,    None,    Some(8), None,     None,     None,     Some(7),  None    ],
        [None,    None,    None,    None,    Some(1),  None,     Some(9),  None,     Some(4) ],
        [Some(1), None,    None,    Some(4), None,     None,     None,     None,     Some(3) ],
        [None,    None,    None,    None,    None,     None,     None,     None,     None    ],
        [None,    None,    Some(7), None,    None,     None,     Some(8),  Some(6),  None    ],
        [None,    None,    Some(6), Some(7), None,     Some(8),  Some(2),  None,     None    ],
        [None,    Some(1), None,    None,    Some(9),  None,     None,     None,     Some(5) ],
        [None,    None,    None,    None,    None,     Some(1),  None,     None,     Some(8) ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    let (solution, stats) = find_one_solution_rand(&puzzle);
    assert!(solution.is_some(), "Puzzle should have a solution");
    assert!(stats.solutions_found == 1);

    // Optional: verify the solution is actually valid
    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
        println!("{}", solved_puzzle);

        stats.print_analysis();
    }
}

#[test]
fn test_shultz_301_ord() {
    #[rustfmt::skip]
    let preset = [
        [None,    Some(3), Some(9), Some(5), None,     None,     None,     None,     None    ],
        [None,    None,    None,    Some(8), None,     None,     None,     Some(7),  None    ],
        [None,    None,    None,    None,    Some(1),  None,     Some(9),  None,     Some(4) ],
        [Some(1), None,    None,    Some(4), None,     None,     None,     None,     Some(3) ],
        [None,    None,    None,    None,    None,     None,     None,     None,     None    ],
        [None,    None,    Some(7), None,    None,     None,     Some(8),  Some(6),  None    ],
        [None,    None,    Some(6), Some(7), None,     Some(8),  Some(2),  None,     None    ],
        [None,    Some(1), None,    None,    Some(9),  None,     None,     None,     Some(5) ],
        [None,    None,    None,    None,    None,     Some(1),  None,     None,     Some(8) ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    // Specify row & col orders
    let row_order: [usize; 9] = [2, 5, 1, 6, 3, 7, 4, 8, 0]; // Some orders don't reach a soln?
    let col_order: [usize; 9] = [6, 8, 3, 4, 2, 0, 7, 5, 1];

    let (solution, stats) = find_one_solution_ord(&puzzle, Some(row_order), Some(col_order));
    assert!(solution.is_some(), "Puzzle should have a solution");
    assert!(stats.solutions_found == 1);

    // Optional: verify the solution is actually valid
    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
        println!("{}", solved_puzzle);

        stats.print_analysis();
    }
}

#[test]
fn test_shultz_301_all_soln() {
    #[rustfmt::skip]
    let preset = [
        [None,    Some(3), Some(9), Some(5), None,     None,     None,     None,     None    ],
        [None,    None,    None,    Some(8), None,     None,     None,     Some(7),  None    ],
        [None,    None,    None,    None,    Some(1),  None,     Some(9),  None,     Some(4) ],
        [Some(1), None,    None,    Some(4), None,     None,     None,     None,     Some(3) ],
        [None,    None,    None,    None,    None,     None,     None,     None,     None    ],
        [None,    None,    Some(7), None,    None,     None,     Some(8),  Some(6),  None    ],
        [None,    None,    Some(6), Some(7), None,     Some(8),  Some(2),  None,     None    ],
        [None,    Some(1), None,    None,    Some(9),  None,     None,     None,     Some(5) ],
        [None,    None,    None,    None,    None,     Some(1),  None,     None,     Some(8) ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    let (solutions, stats) = find_all_solutions(&puzzle);

    // Print comprehensive analysis
    stats.print_analysis();

    // Print all solutions to verify they're different
    println!("\nAll solutions found:");
    for (i, solution) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        println!("{}", solution);
        assert!(solution.is_solved(), "Solution {} should be valid", i + 1);

        // Optional: Print a separator between solutions
        if i < solutions.len() - 1 {
            println!("{}", "-".repeat(30));
        }
    }
}

#[test]
fn test_mepham_d() {
    #[rustfmt::skip]
    let preset = [
        [None,    Some(9), None,    Some(7), None,     None,     Some(8), Some(6), None    ],
        [None,    Some(3), Some(1), None,    None,     Some(5),  None,    Some(2), None    ],
        [Some(8), None,    Some(6), None,    None,     None,     None,    None,    None    ],
        [None,    None,    Some(7), None,    Some(5),  None,     None,    None,    Some(6) ],
        [None,    None,    None,    Some(3), None,     Some(7),  None,    None,    None    ],
        [Some(5), None,    None,    None,    Some(1),  None,     Some(7), None,    None    ],
        [None,    None,    None,    None,    None,     None,     Some(1), None,    Some(9) ],
        [None,    Some(2), None,    Some(6), None,     None,     Some(3), Some(5), None    ],
        [None,    Some(5), Some(4), None,    None,     Some(8),  None,    Some(7), None    ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    let (solution, stats) = find_one_solution(&puzzle);
    assert!(solution.is_some(), "Puzzle should have a solution");
    assert!(stats.solutions_found == 1);

    // Optional: verify the solution is actually valid
    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
        println!("{}", solved_puzzle);

        stats.print_analysis();
    }
}

#[test]
fn test_mepham_d_all_soln() {
    #[rustfmt::skip]
    let preset = [
        [None,    Some(9), None,    Some(7), None,     None,     Some(8), Some(6), None    ],
        [None,    Some(3), Some(1), None,    None,     Some(5),  None,    Some(2), None    ],
        [Some(8), None,    Some(6), None,    None,     None,     None,    None,    None    ],
        [None,    None,    Some(7), None,    Some(5),  None,     None,    None,    Some(6) ],
        [None,    None,    None,    Some(3), None,     Some(7),  None,    None,    None    ],
        [Some(5), None,    None,    None,    Some(1),  None,     Some(7), None,    None    ],
        [None,    None,    None,    None,    None,     None,     Some(1), None,    Some(9) ],
        [None,    Some(2), None,    Some(6), None,     None,     Some(3), Some(5), None    ],
        [None,    Some(5), Some(4), None,    None,     Some(8),  None,    Some(7), None    ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    let (solutions, stats) = find_all_solutions(&puzzle);

    // Print comprehensive analysis
    stats.print_analysis();

    // Print all solutions to verify they're different
    println!("\nAll solutions found:");
    for (i, solution) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        println!("{}", solution);
        assert!(solution.is_solved(), "Solution {} should be valid", i + 1);

        // Optional: Print a separator between solutions
        if i < solutions.len() - 1 {
            println!("{}", "-".repeat(30));
        }
    }
}

#[test]
fn test_murty_2_multiple_solutions() {
    #[rustfmt::skip]
    let preset = [
        [Some(9), None,    Some(6), None,    Some(7), None,    Some(4), None,    Some(3)],
        [None,    None,    None,    Some(4), None,    None,    Some(2), None,    None   ],
        [None,    Some(7), None,    None,    Some(2), Some(3), None,    Some(1), None   ],
        [Some(5), None,    None,    None,    None,    None,    Some(1), None,    None   ],
        [None,    Some(4), None,    Some(2), None,    Some(8), None,    Some(6), None   ],
        [None,    None,    Some(3), None,    None,    None,    None,    None,    Some(5)],
        [None,    Some(3), None,    Some(7), None,    None,    None,    Some(5), None   ],
        [None,    None,    Some(7), None,    None,    Some(5), None,    None,    None   ],
        [Some(4), None,    Some(5), None,    Some(1), None,    Some(7), None,    Some(8)],
    ];

    let puzzle = Sudoku::from_preset(preset);

    let (solutions, stats) = find_all_solutions(&puzzle);

    // This puzzle should have multiple solutions
    assert!(solutions.len() > 1, "Puzzle should have multiple solutions");
    println!("Found {} solutions", solutions.len());

    // Print comprehensive analysis
    stats.print_analysis();

    // Print all solutions to verify they're different
    println!("\nAll solutions found:");
    for (i, solution) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        println!("{}", solution);
        assert!(solution.is_solved(), "Solution {} should be valid", i + 1);

        // Optional: Print a separator between solutions
        if i < solutions.len() - 1 {
            println!("{}", "-".repeat(30));
        }
    }
}

#[test]
fn test_is_safe_function() {
    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut subgrids = [0u16; 9];

    // Place number 5 at position (0,0)
    rows[0] |= 1 << 5;
    cols[0] |= 1 << 5;
    subgrids[0] |= 1 << 5;

    // Should not be safe to place 5 again in same row/col/subgrid
    assert!(!is_safe(&rows, &cols, &subgrids, 0, 1, 5));
    assert!(!is_safe(&rows, &cols, &subgrids, 1, 0, 5));
    assert!(!is_safe(&rows, &cols, &subgrids, 1, 1, 5));

    // Should be safe to place different number
    assert!(is_safe(&rows, &cols, &subgrids, 0, 1, 6));
}

#[test]
fn test_tree_width_tracking() {
    let puzzle = Sudoku::new();
    let (solution, stats) = find_one_solution(&puzzle);

    assert!(solution.is_some());

    // These should still hold true
    let total_nodes: usize = stats.tree_width_by_level.iter().sum();
    assert_eq!(total_nodes, stats.nodes_explored);
    assert_eq!(stats.tree_width_by_level[0], 1);

    // Max depth should now be 80, not 81
    assert!(stats.max_recursion_depth <= 80);

    // Optional: verify the solution is actually valid
    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
        println!("{}", solved_puzzle)
    }

    println!("Tree width by level:");
    for (depth, width) in stats.tree_width_by_level.iter().enumerate() {
        if *width > 0 {
            println!("Depth {}: {} nodes", depth, width);
        }
    }
}

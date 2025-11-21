use sudoku_tool::core::solvers::bf_solver::*;
use sudoku_tool::core::sudoku::Sudoku;

#[test]
fn test_solve_empty_puzzle() {
    let puzzle = Sudoku::new();
    let (solution, stats) = find_one_solution(&puzzle);

    assert!(solution.is_some(), "Empty puzzle should have a solution");
    assert!(stats.solutions_found == 1);
    assert!(stats.nodes_explored > 0);
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
    assert!(
        stats.nodes_explored <= 81,
        "Solved puzzle should require minimal exploration"
    );
}

#[test]
fn test_shultz_301_all_strategies() {
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

    // Test all strategies
    let strategies = [
        ("Default", SearchStrategy::Default),
        ("Row/Col Random", SearchStrategy::RowColRandom),
        ("Custom Row/Col", SearchStrategy::CustomRowCol { 
            row_order: [2, 5, 1, 6, 3, 7, 4, 8, 0], 
            col_order: [6, 8, 3, 4, 2, 0, 7, 5, 1] 
        }),
    ];

    for (name, strategy) in strategies {
        println!("Testing {} strategy...", name);
        let (solution, stats) = find_one_solution_strategy(&puzzle, strategy.clone());
        assert!(solution.is_some(), "{} strategy should find a solution", name);
        assert!(stats.solutions_found == 1);

        if let Some(solved_puzzle) = solution {
            assert!(solved_puzzle.is_solved(), "{} strategy solution should be valid", name);
            println!("{} strategy: {} nodes explored", name, stats.nodes_explored);
        }
    }
}

#[test]
fn test_cell_order_strategies_simple() {
    // Use a simpler puzzle for cell ordering tests
    #[rustfmt::skip]
    let preset = [
        [None,    Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)],
        [Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3)],
        [Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
        [Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1)],
        [Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4)],
        [Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)],
        [Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)],
        [Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5)],
        [Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None   ],
    ];

    let puzzle = Sudoku::from_preset(preset);

    // Test cell-based strategies on simple puzzle
    let strategies = [
        ("Cell Random", SearchStrategy::CellRandom),
        ("Custom Cell", SearchStrategy::CustomCell { 
            cell_order: {
                let mut cells = Vec::new();
                for i in (0..9).rev() {
                    for j in (0..9).rev() {
                        cells.push((i, j));
                    }
                }
                cells
            }
        }),
    ];

    for (name, strategy) in strategies {
        println!("Testing {} strategy on simple puzzle...", name);
        let (solution, stats) = find_one_solution_strategy(&puzzle, strategy.clone());
        assert!(solution.is_some(), "{} strategy should find a solution for simple puzzle", name);
        
        if let Some(solved_puzzle) = solution {
            assert!(solved_puzzle.is_solved(), "{} strategy solution should be valid", name);
            println!("{} strategy: {} nodes explored", name, stats.nodes_explored);
        }
    }
}

#[test]
fn test_cell_order_generation() {
    // Test that cell order generation works correctly
    let cell_order = generate_cell_order_from_row_col(&[0, 1, 2, 3, 4, 5, 6, 7, 8], &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
    
    assert_eq!(cell_order.len(), 81, "Cell order should have 81 elements");
    assert_eq!(cell_order[0], (0, 0), "First cell should be (0, 0)");
    assert_eq!(cell_order[1], (0, 1), "Second cell should be (0, 1)");
    assert_eq!(cell_order[80], (8, 8), "Last cell should be (8, 8)");
    
    println!("Cell order generation test passed!");
}

#[test]
fn test_wrapper_functions_fast() {
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

    // Test the fast wrapper functions (excluding random cell order)
    let (solution1, _) = find_one_solution(&puzzle);
    assert!(solution1.is_some(), "Default wrapper should work");
    
    let (solution2, _) = find_one_solution_rand_rowcol_order(&puzzle);
    assert!(solution2.is_some(), "Row/Col random wrapper should work");

    // Test custom row/col wrapper
    let row_order = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let col_order = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let (solution3, _) = find_one_solution_custom_rowcol_order(&puzzle, row_order, col_order);
    assert!(solution3.is_some(), "Custom row/col wrapper should work");

    // Test custom cell wrapper (non-random)
    let cell_order: Vec<(usize, usize)> = (0..9)
        .flat_map(|i| (0..9).map(move |j| (i, j)))
        .collect();
    let (solution4, _) = find_one_solution_custom_cell_order(&puzzle, &cell_order);
    assert!(solution4.is_some(), "Custom cell wrapper should work");

    // Verify all solutions are valid
    for (i, solution) in [solution1, solution2, solution3, solution4].iter().enumerate() {
        if let Some(solved_puzzle) = solution {
            assert!(solved_puzzle.is_solved(), "Solution {} should be valid", i + 1);
        }
    }

    println!("All fast wrapper functions work correctly!");
}

#[test]
fn test_wrapper_function_random_cell_order() {
    // Use a simpler puzzle for the slow random cell order test
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

    println!("Testing random cell order wrapper (this may take a while)...");
    let (solution, stats) = find_one_solution_rand_cell_order(&puzzle);
    assert!(solution.is_some(), "Random cell order wrapper should work");
    
    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Random cell order solution should be valid");
        println!("Random cell order wrapper succeeded with {} nodes explored", stats.nodes_explored);
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

    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
        println!("Mepham D solved with {} nodes explored", stats.nodes_explored);
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

    // Verify tree width tracking is consistent
    let total_nodes: usize = stats.tree_width_by_level.iter().sum();
    assert_eq!(total_nodes, stats.nodes_explored);
    assert_eq!(stats.tree_width_by_level[0], 1);
    assert!(stats.max_recursion_depth <= 80);

    if let Some(solved_puzzle) = solution {
        assert!(solved_puzzle.is_solved(), "Solution should be valid");
    }

    println!("Tree width tracking test passed with {} total nodes", stats.nodes_explored);
}

#[test]
fn test_strategy_performance_comparison() {
    // Use a KNOWN solvable, medium difficulty puzzle
    #[rustfmt::skip]
    let preset = [
        [None,    None,    Some(3), None,    Some(2), None,    Some(6), None,    None],
        [Some(9), None,    None,    Some(3), None,    Some(5), None,    None,    Some(1)],
        [None,    None,    Some(1), Some(8), None,    Some(6), Some(4), None,    None],
        [None,    None,    Some(8), Some(1), None,    Some(2), Some(9), None,    None],
        [Some(7), None,    None,    None,    None,    None,    None,    None,    Some(8)],
        [None,    None,    Some(6), Some(7), None,    Some(8), Some(2), None,    None],
        [None,    None,    Some(2), Some(6), None,    Some(9), Some(5), None,    None],
        [Some(8), None,    None,    Some(2), None,    Some(3), None,    None,    Some(9)],
        [None,    None,    Some(5), None,    Some(1), None,    Some(3), None,    None],
    ];

    let puzzle = Sudoku::from_preset(preset);

    // First, verify the puzzle is valid and solvable with default strategy
    println!("\n=== Verifying puzzle is solvable ===");
    let (default_solution, default_stats) = find_one_solution(&puzzle);
    
    if default_solution.is_none() {
        println!("ERROR: Puzzle is not solvable with default strategy!");
        println!("Puzzle:");
        println!("{}", puzzle);
        panic!("Test puzzle is not solvable");
    }

    println!("Puzzle verified as solvable with default strategy");
    println!("Default: {} nodes, {:?}", default_stats.nodes_explored, default_stats.search_duration);

    let strategies = [
        ("Default", SearchStrategy::Default),
        ("Row/Col Random", SearchStrategy::RowColRandom),
    ];

    println!("\n=== Performance Comparison ===");
    for (name, strategy) in strategies {
        println!("Testing {} strategy...", name);
        let (solution, stats) = find_one_solution_strategy(&puzzle, strategy.clone());
        
        if solution.is_none() {
            println!("ERROR: {} strategy failed to solve the puzzle!", name);
            println!("This suggests a bug in the strategy implementation");
            panic!("{} strategy failed", name);
        }
        
        println!("{}: {} nodes, {:?}", name, stats.nodes_explored, stats.search_duration);
        
        if let Some(solved_puzzle) = solution {
            assert!(solved_puzzle.is_solved(), "{} strategy solution should be valid", name);
        }
    }
}
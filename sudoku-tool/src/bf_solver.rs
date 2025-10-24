use crate::sudoku::Sudoku;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SolverStats {
    pub solutions_found: usize,
    pub search_duration: Duration,
    pub max_recursion_depth: usize,
    pub nodes_explored: usize,
    pub backtracks: usize,
    pub tree_width_by_level: [usize; 81], // Vec that stores width at each depth level (index)
}

impl Default for SolverStats {
    fn default() -> Self {
        Self {
            solutions_found: 0,
            search_duration: Duration::default(),
            max_recursion_depth: 0,
            nodes_explored: 0,
            backtracks: 0,
            tree_width_by_level: [0; 81],
        }
    }
}

impl SolverStats {
    /// Print comprehensive analysis of solver performance and search tree
    pub fn print_analysis(&self) {
        println!("=== Sudoku Search Tree Analysis ===");

        let (max_width, max_width_depth) = self.max_tree_width();
        let total_nodes = self.total_nodes_from_tree();

        println!("Summary:");
        println!("  Solutions found: {}", self.solutions_found);
        println!("  Search duration: {:?}", self.search_duration);
        println!("  Total nodes explored: {}", self.nodes_explored);
        println!(
            "  Sum of tree widths: {} (verification: {})",
            total_nodes,
            if self.is_tree_data_consistent() {
                "PASS"
            } else {
                "FAIL"
            }
        );
        println!(
            "  Maximum tree width: {} at depth {}",
            max_width, max_width_depth
        );
        println!("  Maximum recursion depth: {}", self.max_recursion_depth);
        println!("  Backtracks: {}", self.backtracks);
        println!("  Branching levels: {}", self.branching_levels_count());
        println!(
            "  Avg branching factor: {:.2}",
            self.average_branching_factor()
        );

        self.print_tree_bar_chart();

        println!("=====================================");
    }

    /// Get the maximum tree width and its depth
    pub fn max_tree_width(&self) -> (usize, usize) {
        let max_width = self.tree_width_by_level.iter().max().unwrap_or(&0);
        let depth = self
            .tree_width_by_level
            .iter()
            .position(|&w| w == *max_width)
            .unwrap_or(0);
        (*max_width, depth)
    }

    /// Get the total number of nodes from tree width data (for verification)
    pub fn total_nodes_from_tree(&self) -> usize {
        self.tree_width_by_level.iter().sum()
    }

    /// Check if tree width data is consistent with nodes explored count
    pub fn is_tree_data_consistent(&self) -> bool {
        self.total_nodes_from_tree() == self.nodes_explored
    }

    /// Get the average branching factor
    pub fn average_branching_factor(&self) -> f64 {
        let branching_levels = self.branching_levels_count();
        if branching_levels > 0 {
            self.nodes_explored as f64 / branching_levels as f64
        } else {
            0.0
        }
    }

    /// Get the number of levels with actual branching
    pub fn branching_levels_count(&self) -> usize {
        self.tree_width_by_level.iter().filter(|&&w| w > 0).count()
    }

    /// Print tree width distribution (non-zero only)
    pub fn print_tree_widths(&self) {
        println!("\nTree width by level (non-zero only):");
        for (depth, width) in self.tree_width_by_level.iter().enumerate() {
            if *width > 0 {
                println!("  Depth {:2}: {:4} nodes", depth, width);
            }
        }
    }

    /// Print polished bar chart with perfect alignment
    pub fn print_tree_bar_chart(&self) {
        let non_zero_levels = self.non_zero_tree_widths();
        if non_zero_levels.is_empty() {
            println!("\nTree Width Distribution: (no data)");
            return;
        }

        let (max_width, _) = self.max_tree_width();
        println!("\nTree Width Distribution (Bar Chart):");

        let max_bar_width = 50;
        let scale_factor = max_bar_width as f64 / max_width as f64;

        // Calculate the maximum width needed for numbers
        let max_num_width = non_zero_levels
            .iter()
            .map(|(_, w)| format!("{}", w).len())
            .max()
            .unwrap_or(1);

        for (depth, width) in non_zero_levels {
            let bar_length = (width as f64 * scale_factor).max(1.0) as usize; // At least 1 for visibility
            let bar = "█".repeat(bar_length);

            // Perfect alignment using fixed-width number formatting
            println!(
                "  Depth {:2}: {:>num_width$} {}",
                depth,
                width,
                bar,
                num_width = max_num_width
            );
        }

        // Add scale reference
        println!(
            "\n  Scale: 1█ ≈ {:.0} nodes",
            max_width as f64 / max_bar_width as f64
        );
    }

    /// Get a vector of (depth, width) pairs for non-zero levels only
    pub fn non_zero_tree_widths(&self) -> Vec<(usize, usize)> {
        self.tree_width_by_level
            .iter()
            .enumerate()
            .filter_map(|(depth, &w)| if w > 0 { Some((depth, w)) } else { None })
            .collect()
    }
}

// Finds a solution to a Sudoku puzzle
pub fn find_one_solution(sudoku: &Sudoku) -> (Option<Sudoku>, SolverStats) {
    // Initialize stat recorders
    let start_time = Instant::now();
    let mut stats = SolverStats::default();
    let mut solutions = Vec::<Sudoku>::new(); // We'll use this to get the first solution

    // Instantiates board, row, col, and subgrid data structures
    let mut board = [[0u8; 9]; 9];
    let (mut rows, mut cols, mut subgrids) = ([0u16; 9], [0u16; 9], [0u16; 9]);

    initialize_from_sudoku(sudoku, &mut board, &mut rows, &mut cols, &mut subgrids);

    solve_recursive(
        &mut board,
        &mut rows,
        &mut cols,
        &mut subgrids,
        0,
        0,
        0,
        &mut stats,
        &mut solutions,
        false,
    );

    let solution = solutions.into_iter().next(); // Take the first solution if any

    stats.solutions_found = if solution.is_some() { 1 } else { 0 };
    stats.search_duration = start_time.elapsed();
    (solution, stats)
}

// Finds all solutions to a Sudoku puzzle
pub fn find_all_solutions(sudoku: &Sudoku) -> (Vec<Sudoku>, SolverStats) {
    // Initialize stat recorders and solutions vec
    let start_time = Instant::now();
    let mut stats = SolverStats::default();
    let mut solutions = Vec::new();

    // Instantiates board, row, col, and subgrid data structures
    let mut board = [[0u8; 9]; 9];
    let (mut rows, mut cols, mut subgrids) = ([0u16; 9], [0u16; 9], [0u16; 9]);

    // Initializes from original puzzle and it is read-only
    initialize_from_sudoku(sudoku, &mut board, &mut rows, &mut cols, &mut subgrids);

    solve_recursive(
        &mut board,
        &mut rows,
        &mut cols,
        &mut subgrids,
        0,
        0,
        0,
        &mut stats,
        &mut solutions,
        true,
    );

    stats.solutions_found = solutions.len();
    stats.search_duration = start_time.elapsed();
    (solutions, stats)
}

// Initializes board, row, col, and subgrid data structures
fn initialize_from_sudoku(
    sudoku: &Sudoku,
    board: &mut [[u8; 9]; 9],
    rows: &mut [u16; 9],
    cols: &mut [u16; 9],
    subgrids: &mut [u16; 9],
) {
    for i in 0..9 {
        for j in 0..9 {
            if let Some(value) = sudoku.get_solved_value(i, j) {
                board[i][j] = value;
                let bit = 1 << value; // bitwise left shift so that the value-th bit is set to 1
                rows[i] |= bit; // bitwise OR operator updates rows[i] with the information from bit
                cols[j] |= bit;
                subgrids[(i / 3) * 3 + j / 3] |= bit;
            }
        }
    }
}

fn solve_recursive(
    board: &mut [[u8; 9]; 9],
    rows: &mut [u16; 9],
    cols: &mut [u16; 9],
    subgrids: &mut [u16; 9],
    i: usize,
    j: usize,
    depth: usize,
    stats: &mut SolverStats,
    solutions: &mut Vec<Sudoku>,
    find_all: bool,
) {
    // Find next empty cell
    let (mut i, mut j) = (i, j);
    while i < 9 && board[i][j] != 0 {
        j += 1;
        if j == 9 {
            j = 0;
            i += 1;
        }
    }

    // Check if board is filled
    if i == 9 {
        let mut solution_sudoku = Sudoku::new();
        // Copy solution to Sudoku struct
        for row in 0..9 {
            for col in 0..9 {
                solution_sudoku.set_cell(row, col, board[row][col]).unwrap();
            }
        }
        solutions.push(solution_sudoku);

        return;
    }

    // Update depth and node stats
    stats.nodes_explored += 1;
    stats.max_recursion_depth = stats.max_recursion_depth.max(depth);
    stats.tree_width_by_level[depth] += 1;

    for num in 1..=9 {
        if is_safe(rows, cols, subgrids, i, j, num) {
            // Place number
            board[i][j] = num;

            // Update the u16 bits in each row, col, and subgrid
            let bit = 1 << num;
            rows[i] |= bit;
            cols[j] |= bit;
            subgrids[(i / 3) * 3 + j / 3] |= bit;

            // Calculate next cell to call recursively
            let next_j = if j == 8 { 0 } else { j + 1 }; // if at the end of the row, move to beginning of next row
            let next_i = if j == 8 { i + 1 } else { i };

            solve_recursive(
                board,
                rows,
                cols,
                subgrids,
                next_i,
                next_j,
                depth + 1,
                stats,
                solutions,
                find_all,
            );

            // If we found at least one solution and we're not finding all, early return
            if !solutions.is_empty() && !find_all {
                return;
            }

            // Backtrack
            board[i][j] = 0; // Set current cell to 0
            rows[i] &= !bit; // Flips the num-th bit (current cell) to 0
            cols[j] &= !bit;
            subgrids[(i / 3) * 3 + j / 3] &= !bit;
            stats.backtracks += 1;
        }
    }
}

fn is_safe(
    rows: &[u16; 9],
    cols: &[u16; 9],
    subgrids: &[u16; 9],
    i: usize,
    j: usize,
    num: u8,
) -> bool {
    let bit = 1 << num;
    /*
       bit is a u16 where every bit is 0 except for the bit in the num-th position.

       (rows[i] & bit) == 0 checks if the num-th position in rows[i] is 0.
           (rows[i] & bit) is only 0 if the num-th bit in rows[i] is 0.
           Otherwise, (rows[i] & bit) = bit.
       If it is, this returns true.
       If not, it returns false, meaning that the cell is not safe.
    */
    (rows[i] & bit) == 0 && (cols[j] & bit) == 0 && (subgrids[(i / 3) * 3 + j / 3] & bit) == 0
}

/*
    Unit Tests!!
*/
#[cfg(test)]
mod tests {
    use super::*;

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
}

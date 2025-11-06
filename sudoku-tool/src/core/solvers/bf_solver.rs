use crate::core::sudoku::Sudoku;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SolverStats {
    pub solutions_found: usize,
    pub search_duration: Duration,
    pub max_recursion_depth: usize,
    pub nodes_explored: usize,
    pub backtracks: usize,
    pub leaves: usize,
    pub tree_width_by_level: [usize; 81],
}

impl Default for SolverStats {
    fn default() -> Self {
        Self {
            solutions_found: 0,
            search_duration: Duration::default(),
            max_recursion_depth: 0,
            nodes_explored: 0,
            backtracks: 0,
            leaves: 0,
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
            "  Total leaves: {} ({} solutions, {} dead ends)",
            self.leaves,
            self.solutions_found,
            self.dead_end_leaves()
        );
        println!("  Leaf ratio: {:.2}%", self.leaf_ratio() * 100.0);
        println!(
            "  Solution leaf percentage: {:.6}%",
            self.solution_leaf_percentage()
        );
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
        // println!("  Branching levels: {}", self.branching_levels_count());
        // println!(
        //     "  Avg branching factor: {:.2}",
        //     self.average_branching_factor()
        // );

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

    /// Get the number of dead-end leaves (positions with no valid moves)
    pub fn dead_end_leaves(&self) -> usize {
        self.leaves.saturating_sub(self.solutions_found)
    }

    /// Get the leaf-to-node ratio (higher means more pruning)
    pub fn leaf_ratio(&self) -> f64 {
        if self.nodes_explored > 0 {
            self.leaves as f64 / self.nodes_explored as f64
        } else {
            0.0
        }
    }

    /// Get the percentage of leaves that are solutions
    pub fn solution_leaf_percentage(&self) -> f64 {
        if self.leaves > 0 {
            self.solutions_found as f64 / self.leaves as f64 * 100.0
        } else {
            0.0
        }
    }
}

// Finds a solution to a Sudoku puzzle
pub fn find_one_solution(sudoku: &Sudoku) -> (Option<Sudoku>, SolverStats) {
    find_one_solution_ord(sudoku, None, None)
}

pub fn find_one_solution_rand(sudoku: &Sudoku) -> (Option<Sudoku>, SolverStats) {
    // Generate random sequence of 0-8
    let mut row_arr: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut col_arr: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    row_arr.shuffle(&mut rand::rng());
    col_arr.shuffle(&mut rand::rng());

    println!("{:?}", row_arr);
    println!("{:?}", col_arr);

    find_one_solution_ord(sudoku, Some(row_arr), Some(col_arr))
}

// Finds a solution to a Sudoku puzzle with a random order
pub fn find_one_solution_ord(
    sudoku: &Sudoku,
    row_order: Option<[usize; 9]>,
    col_order: Option<[usize; 9]>,
) -> (Option<Sudoku>, SolverStats) {
    // Initialize stat recorders
    let start_time = Instant::now();
    let mut stats = SolverStats::default();
    let mut solutions = Vec::<Sudoku>::new(); // We'll use this to get the first solution

    // Instantiates board, row, col, and subgrid data structures
    let mut board = [[0u8; 9]; 9];
    let (mut rows, mut cols, mut subgrids) = ([0u16; 9], [0u16; 9], [0u16; 9]);

    initialize_from_sudoku(sudoku, &mut board, &mut rows, &mut cols, &mut subgrids);

    // Use provided order or default to 0..8
    let row_order = row_order.unwrap_or([0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let col_order = col_order.unwrap_or([0, 1, 2, 3, 4, 5, 6, 7, 8]);

    solve_recursive(
        &mut board,
        &mut rows,
        &mut cols,
        &mut subgrids,
        &row_order,
        &col_order,
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

    let row_order: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let col_order: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    solve_recursive(
        &mut board,
        &mut rows,
        &mut cols,
        &mut subgrids,
        &row_order,
        &col_order,
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
    row_order: &[usize; 9],
    col_order: &[usize; 9],
    row_idx: usize, // Index into row_order from 0
    col_idx: usize, // Index into col_order from 0
    depth: usize,
    stats: &mut SolverStats,
    solutions: &mut Vec<Sudoku>,
    find_all: bool,
) {
    // Check if board is filled: REDUNDANT, MAYBE REFACTOR??
    if row_idx == 9 {
        let mut solution_sudoku = Sudoku::new();
        // Copy solution to Sudoku struct
        for row in 0..9 {
            for col in 0..9 {
                solution_sudoku.set_cell(row, col, board[row][col]).unwrap();
            }
        }

        solutions.push(solution_sudoku);

        stats.leaves += 1; // Solution leaf
        return;
    }

    // Mutable indices for row order
    let (mut r_idx, mut c_idx) = (row_idx, col_idx);
    // Convert ordering indices to board coordinates
    let mut i = row_order[r_idx];
    let mut j = col_order[c_idx];

    // Find next empty cell
    while r_idx < 9 && board[i][j] != 0 {
        c_idx += 1;

        if c_idx == 9 {
            // if at end of row, jump to start of next row
            c_idx = 0;
            r_idx += 1;
        }

        if r_idx < 9 {
            i = row_order[r_idx];
            j = col_order[c_idx];
        }
    }

    // Check if board is filled
    if r_idx == 9 {
        let mut solution_sudoku = Sudoku::new();
        // Copy solution to Sudoku struct
        for row in 0..9 {
            for col in 0..9 {
                solution_sudoku.set_cell(row, col, board[row][col]).unwrap();
            }
        }

        solutions.push(solution_sudoku);

        stats.leaves += 1; // Solution leaf
        return;
    }

    // Update depth and node stats
    stats.nodes_explored += 1;
    stats.max_recursion_depth = stats.max_recursion_depth.max(depth);
    stats.tree_width_by_level[depth] += 1;

    let mut any_valid_moves = false;

    for num in 1..=9 {
        if is_safe(rows, cols, subgrids, i, j, num) {
            any_valid_moves = true;

            // Place number
            board[i][j] = num;

            // Update the u16 bits in each row, col, and subgrid
            let bit = 1 << num;
            rows[i] |= bit;
            cols[j] |= bit;
            subgrids[(i / 3) * 3 + j / 3] |= bit;

            // Calculate next cell to call recursively
            let next_c_idx = if c_idx == 8 { 0 } else { c_idx + 1 }; // if at the end of the row, move to beginning of next row
            let next_r_idx = if c_idx == 8 { r_idx + 1 } else { r_idx };

            solve_recursive(
                board,
                rows,
                cols,
                subgrids,
                row_order,
                col_order,
                next_r_idx,
                next_c_idx,
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

    if !any_valid_moves {
        // Dead-end leaf
        stats.leaves += 1;
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

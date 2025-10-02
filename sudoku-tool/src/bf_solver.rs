use crate::sudoku::Sudoku;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct SolverStats {
    pub solutions_found: usize,
    pub search_duration: Duration,
    pub max_recursion_depth: usize,
    pub nodes_explored: usize,
    pub backtracks: usize,
}

// Finds a solution to a Sudoku puzzle
pub fn solve(sudoku: &mut Sudoku) -> (bool, SolverStats) {
    // Initialize stat recorders
    let start_time = Instant::now();
    let mut stats = SolverStats::default();

    // Instantiates board, row, col, and subgrid data structures
    let mut board = [[0u8; 9]; 9];
    let (mut rows, mut cols, mut subgrids) = ([0u16; 9], [0u16; 9], [0u16; 9]);

    initialize_from_sudoku(sudoku, &mut board, &mut rows, &mut cols, &mut subgrids);

    let result = solve_recursive(
        &mut board,
        &mut rows,
        &mut cols,
        &mut subgrids,
        0,
        0,
        0,
        &mut stats,
    );
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
) -> bool {
    // Update depth and node stats
    stats.nodes_explored += 1;
    stats.max_recursion_depth = stats.max_recursion_depth.max(depth);

    let (mut i, mut j) = (i, j);

    // Find next empty cell
    while i < 9 && board[i][j] != 0 {
        j += 1;
        if j == 9 {
            j = 0;
            i += 1;
        }
        if i == 9 {
            return true; // Solution found - we've filled all cells
        }
    }

    for num in 1..=9 {
        todo("implement is_safe()");
        if is_safe(rows, cols, subgrids, i, j, num) {
            // Place number

            // Recursively solve from next position

            // Backtrack
        }
    }

    false
}
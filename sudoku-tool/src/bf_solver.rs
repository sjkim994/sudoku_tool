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
pub fn find_one_solution(sudoku: &mut Sudoku) -> (Option<Sudoku>, SolverStats) {
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
    }

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

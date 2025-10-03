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
        let mut puzzle = Sudoku::new();
        // Set up a known valid solution
        let solved_board = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        for i in 0..9 {
            for j in 0..9 {
                puzzle.set_cell(i, j, solved_board[i][j]).unwrap();
            }
        }

        let (solution, stats) = find_one_solution(&puzzle);
        assert!(
            solution.is_some(),
            "Already solved puzzle should return a solution"
        );
        // Should find solution very quickly (minimal nodes explored)
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
            println!("{}", solved_puzzle)
        }
    }

    #[test]
    fn test_find_all_solutions_empty() {
        let mut puzzle = Sudoku::new();
        let (solutions, stats) = find_all_solutions(&mut puzzle);

        // Empty puzzle has many solutions
        assert!(solutions.len() > 1);
        assert!(stats.solutions_found > 1);
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
}

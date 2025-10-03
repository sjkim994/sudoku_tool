mod sudoku;
mod bf_solver;

use sudoku::Sudoku;
use bf_solver::{find_one_solution, find_all_solutions};

fn main() {
    println!("Sudoku Solver - Testing Various Puzzles");
    println!("=======================================\n");

    // Test 1: Easy puzzle
    println!("1. Easy Puzzle:");
    #[rustfmt::skip]
    let easy_preset = [
        [Some(5), Some(3), None,     None,    Some(7), None,     None,     None,     None    ],
        [Some(6), None,    None,     Some(1), Some(9), Some(5),  None,     None,     None    ],
        [None,    Some(9), Some(8),  None,    None,    None,     None,     Some(6),  None    ],
        [Some(8), None,    None,     None,    Some(6), None,     None,     None,     Some(3) ],
        [Some(4), None,    None,     Some(8), None,    Some(3),  None,     None,     Some(1) ],
        [Some(7), None,    None,     None,    Some(2), None,     None,     None,     Some(6) ],
        [None,    Some(6), None,     None,    None,    None,     Some(2),  Some(8),  None    ],
        [None,    None,    None,     Some(4), Some(1), Some(9),  None,     None,     Some(5) ],
        [None,    None,    None,     None,    Some(8), None,     None,     Some(7),  Some(9) ],
    ];
    
    let easy_puzzle = Sudoku::from_preset(easy_preset);
    let (solution, stats) = find_one_solution(&easy_puzzle);
    
    if let Some(sol) = &solution {
        println!("Solution found!");
        println!("{}", sol);
    } else {
        println!("No solution found!");
    }
    println!("Stats: {:?}\n", stats);

    // Test 2: Hard puzzle (Schultz 301)
    println!("2. Hard Puzzle (Schultz 301):");
    #[rustfmt::skip]
    let hard_preset = [
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
    
    let hard_puzzle = Sudoku::from_preset(hard_preset);
    let (solution, stats) = find_one_solution(&hard_puzzle);
    
    if let Some(sol) = &solution {
        println!("Solution found!");
        println!("{}", sol);
    } else {
        println!("No solution found!");
    }
    println!("Stats: {:?}\n", stats);

    // Test 3: Diabolical puzzle (Mepham's D)
    println!("3. Diabolical puzzle (Mepham's D):");
    #[rustfmt::skip]
    let diabolical_preset = [
        [None,    Some(9), None,    Some(7), None,     None,    Some(8), Some(6),  None    ],
        [None,    Some(3), Some(1), None,    None,     Some(5), None,    Some(2),  None    ],
        [Some(8), None,    Some(6), None,    None,     None,    None,    None,     None    ],
        [None,    None,    Some(7), None,    Some(5),  None,    None,    None,     Some(6) ],
        [None,    None,    None,    Some(3), None,     Some(7), None,    None,     None    ],
        [Some(5), None,    None,    None,    Some(1),  None,    Some(7), None,     None    ],
        [None,    None,    None,    None,    None,     None,    Some(1), None,     Some(9) ],
        [None,    Some(2), None,    Some(6), None,     None,    Some(3), Some(5),  None    ],
        [None,    Some(5), Some(4), None,    None,     Some(8), None,    Some(7),  None    ],
    ];
    
    let diabolical_puzzle = Sudoku::from_preset(diabolical_preset);
    let (solution, stats) = find_one_solution(&diabolical_puzzle);
    
    if let Some(sol) = &solution {
        println!("Solution found!");
        println!("{}", sol);
    } else {
        println!("No solution found!");
    }
    println!("Stats: {:?}\n", stats);
}
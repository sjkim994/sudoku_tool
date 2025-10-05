mod sudoku;
mod bf_solver;

use sudoku::Sudoku;
use bf_solver::{find_one_solution, find_all_solutions};
use std::path::Path;

fn main() {
    println!("Sudoku Solver - Testing Various Puzzles");
    println!("=======================================\n");

    let puzzle_files = [
        "puzzles/easy.txt",
        "puzzles/shortz_301.txt", 
        "puzzles/mepham_d.txt",
    ];

    for file_path in puzzle_files.iter() {
        println!("Solving: {}", file_path);
        
        match Sudoku::from_file(file_path) {
            Ok(puzzle) => {
                println!("Original puzzle:");
                println!("{}", puzzle);
                
                let (solution, stats) = find_one_solution(&puzzle);
                
                if let Some(sol) = &solution {
                    println!("Solution found!");
                    println!("{}", sol);
                } else {
                    println!("No solution found!");
                }
                println!("Stats: {:?}\n", stats);
            }
            Err(e) => {
                println!("Error loading puzzle from {}: {}\n", file_path, e);
            }
        }
    }

    // Special section for testing multiple solutions
    println!("TESTING MULTIPLE SOLUTION PUZZLES");
    println!("==================================\n");

    let multiple_solution_files = [
        "puzzles/murty_2_soln.txt",
        "puzzles/shortz_301.txt",
        "puzzles/mepham_d.txt",
    ];

    for file_path in multiple_solution_files.iter() {
        println!("Solving (all solutions): {}", file_path);
        
        match Sudoku::from_file(file_path) {
            Ok(puzzle) => {
                println!("Original puzzle:");
                println!("{}", puzzle);
                
                let (solutions, stats) = find_all_solutions(&puzzle);
                println!("Found {} solutions", solutions.len());
                println!("Stats: {:?}", stats);
                
                // Print all solutions for the 2-solution puzzle, first few for others
                if file_path.contains("murty_2_soln") {
                    println!("\nAll {} solutions:", solutions.len());
                    for (i, solution) in solutions.iter().enumerate() {
                        println!("Solution {}:", i + 1);
                        println!("{}", solution);
                    }
                }
                println!();
            }
            Err(e) => {
                println!("Error loading puzzle from {}: {}\n", file_path, e);
            }
        }
    }
}
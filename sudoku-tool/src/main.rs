mod sudoku;
mod bf_solver;

use sudoku::Sudoku;
use bf_solver::{find_one_solution, find_all_solutions};
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;

fn export_stats_to_csv(data: &[String], filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    
    for line in data {
        writeln!(&mut file, "{}", line)?;
    }
    
    println!("\nStatistics exported to: {}", filename);
    Ok(())
}

fn main() {
    println!("Sudoku Solver - Testing Various Puzzles");
    println!("=======================================\n");

    let puzzle_files = [
        "puzzles/printable_sudoku_puzzles/25/01.txt",
        "puzzles/printable_sudoku_puzzles/25/02.txt",
        "puzzles/printable_sudoku_puzzles/25/03.txt",
        "puzzles/printable_sudoku_puzzles/25/04.txt",
        "puzzles/printable_sudoku_puzzles/25/05.txt",
        "puzzles/printable_sudoku_puzzles/25/06.txt",
        "puzzles/printable_sudoku_puzzles/25/07.txt",
        "puzzles/printable_sudoku_puzzles/25/08.txt",
        "puzzles/printable_sudoku_puzzles/25/09.txt",
        "puzzles/printable_sudoku_puzzles/25/10.txt"
    ];

    // Create CSV file for statistics
    let mut stats_data = Vec::new();
    stats_data.push("Puzzle,NodesExplored,Backtracks,MaxRecursionDepth,SearchDurationMicros,SolutionsFound".to_string());

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

                // Add stats to CSV data
                let csv_line = format!(
                    "{},{},{},{},{},{}",
                    file_path,
                    stats.nodes_explored,
                    stats.backtracks,
                    stats.max_recursion_depth,
                    stats.search_duration.as_micros(),
                    stats.solutions_found
                );
                stats_data.push(csv_line);
            }
            Err(e) => {
                println!("Error loading puzzle from {}: {}\n", file_path, e);

                // Add error entry to CSV
                let csv_line = format!(
                    "{},ERROR,ERROR,ERROR,ERROR,ERROR",
                    file_path
                );
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

    // Export statistics to CSV file
    export_stats_to_csv(&stats_data, "sudoku_stats.csv").unwrap();
}
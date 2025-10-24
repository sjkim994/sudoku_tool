mod bf_solver;
mod sudoku;

use bf_solver::{SolverStats, find_all_solutions, find_one_solution};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use sudoku::Sudoku;

#[derive(Debug)]
struct TestConfig {
    one_solution_dir: String,
    all_solutions_dir: String,
    comparison_dir: String,
    output_csv: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            one_solution_dir: "puzzles/single_solution".to_string(),
            all_solutions_dir: "puzzles/multiple_solutions".to_string(),
            comparison_dir: "puzzles/comparison".to_string(),
            output_csv: "sudoku_stats.csv".to_string(),
        }
    }
}

#[derive(Debug)]
struct PuzzleResult {
    filename: String,
    solutions: Vec<Sudoku>, // All solutions found
    stats: SolverStats,
}

impl PuzzleResult {
    fn solution_count(&self) -> usize {
        self.stats.solutions_found
    }

    fn has_solutions(&self) -> bool {
        self.stats.solutions_found != 0
    }

    fn first_solution(&self) -> Option<&Sudoku> {
        self.solutions.first()
    }

    fn display_solutions(&self, max_to_display: Option<usize>) {
        let max_display = max_to_display.unwrap_or(usize::MAX);
        let solutions_to_display = self.solutions.len().min(max_display);

        println!(
            "Found {} solution(s) for {}",
            self.solutions.len(),
            self.filename
        );

        for (i, solution) in self.solutions.iter().take(solutions_to_display).enumerate() {
            println!("Solution {}:", i + 1);
            println!("{}", solution);
            if i < solutions_to_display - 1 {
                println!("{}", "-".repeat(20));
            }
        }

        if self.solutions.len() > max_display {
            println!(
                "... and {} more solutions",
                self.solutions.len() - max_display
            );
        }
    }
}

fn scan_directory_for_puzzles(directory: &str) -> Vec<String> {
    let mut puzzle_files = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "txt") {
                puzzle_files.push(path.to_string_lossy().to_string());
            }
        }
    }

    puzzle_files.sort(); // Sort for consistent ordering
    puzzle_files
}

fn test_puzzles_one_solution(puzzle_files: &[String]) -> Vec<PuzzleResult> {
    let mut results = Vec::new();

    for file_path in puzzle_files {
        println!("Testing (one solution): {}", file_path);

        match Sudoku::from_file(file_path) {
            Ok(puzzle) => {
                let (solution, stats) = find_one_solution(&puzzle);
                let solutions = solution.into_iter().collect(); // Convert Option to Vec

                results.push(PuzzleResult {
                    filename: file_path.clone(),
                    solutions,
                    stats,
                });

                if results.last().unwrap().stats.solutions_found > 0 {
                    println!("  ✓ Solution found");
                } else {
                    println!("  ✗ No solution found");
                }
            }
            Err(e) => {
                eprintln!("Error loading {}: {}", file_path, e);
            }
        }
    }

    results
}

fn test_puzzles_all_solutions(puzzle_files: &[String]) -> Vec<PuzzleResult> {
    let mut results = Vec::new();

    for file_path in puzzle_files {
        println!("Testing (all solutions): {}", file_path);

        match Sudoku::from_file(file_path) {
            Ok(puzzle) => {
                let (solutions, stats) = find_all_solutions(&puzzle);

                results.push(PuzzleResult {
                    filename: file_path.clone(),
                    solutions,
                    stats,
                });

                let solution_count = results.last().unwrap().stats.solutions_found;
                println!("  Found {} solution(s)", solution_count);

                // Display first few solutions for multiple solution puzzles
                if solution_count > 1 {
                    println!("  First 2 solutions:");
                    for (i, solution) in
                        results.last().unwrap().solutions.iter().take(2).enumerate()
                    {
                        println!("  Solution {}:", i + 1);
                        println!("{}", solution);
                    }
                    if solution_count > 2 {
                        println!("  ... and {} more solutions", solution_count - 2);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error loading {}: {}", file_path, e);
            }
        }
    }

    results
}

fn compare_solver_modes(puzzle_files: &[String]) -> Vec<(PuzzleResult, PuzzleResult)> {
    todo!("implement this?");

    let mut comparisons = Vec::new();

    for file_path in puzzle_files {
        println!("Comparing modes for: {}", file_path);

        match Sudoku::from_file(file_path) {
            Ok(puzzle) => {
                let (single_solution, single_stats) = find_one_solution(&puzzle);
                let (all_solutions, all_stats) = find_all_solutions(&puzzle);

                let single_result = PuzzleResult {
                    filename: file_path.clone(),
                    solutions: single_solution.into_iter().collect(),
                    stats: single_stats,
                };

                let all_result = PuzzleResult {
                    filename: file_path.clone(),
                    solutions: all_solutions,
                    stats: all_stats,
                };

                comparisons.push((single_result, all_result));

                // Print comparison summary
                println!(
                    "  Single mode: {} nodes, {} solutions",
                    single_result.stats.nodes_explored,
                    single_result.solution_count()
                );
                println!(
                    "  All mode: {} nodes, {} solutions",
                    all_result.stats.nodes_explored,
                    all_result.solution_count()
                );

                // Verify solutions match for single-solution puzzles
                if single_result.solution_count() == 1 && all_result.solution_count() >= 1 {
                    if let (Some(single_sol), Some(all_sol)) =
                        (single_result.first_solution(), all_result.first_solution())
                    {
                        // if single_sol != all_sol {
                        //     println!("  WARNING: Solutions differ between modes!");
                        // } else {
                        //     println!("  ✓ Solutions match between modes");
                        // }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error loading {}: {}", file_path, e);
            }
        }
    }

    comparisons
}

fn export_results_to_csv(results: &[PuzzleResult], filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;

    // CSV header
    writeln!(
        &mut file,
        "Puzzle,SolutionCount,NodesExplored,Backtracks,Leaves,MaxRecursionDepth,SearchDurationMicros,MaxTreeWidth,MaxTreeWidthDepth,AvgBranchingFactor,HasMultipleSolutions"
    )?;

    for result in results {
        let (max_width, max_width_depth) = result.stats.max_tree_width();
        let has_multiple = if result.solution_count() > 1 {
            "YES"
        } else {
            "NO"
        };

        writeln!(
            &mut file,
            "{},{},{},{},{},{},{},{},{},{:.2},{}",
            result.filename,
            result.solution_count(),
            result.stats.nodes_explored,
            result.stats.backtracks,
            result.stats.leaves,
            result.stats.max_recursion_depth,
            result.stats.search_duration.as_micros(),
            max_width,
            max_width_depth,
            result.stats.average_branching_factor(),
            has_multiple
        )?;
    }

    println!("Results exported to: {}", filename);
    Ok(())
}

fn main() {
    println!("Sudoku Solver - Automated Testing");
    println!("=================================\n");

    let config = TestConfig::default();

    // Create directories if they don't exist
    let _ = fs::create_dir_all(&config.one_solution_dir);
    let _ = fs::create_dir_all(&config.all_solutions_dir);
    let _ = fs::create_dir_all(&config.comparison_dir);

    // Test single solution puzzles
    println!("TESTING SINGLE SOLUTION PUZZLES");
    println!("===============================");
    let single_puzzles = scan_directory_for_puzzles(&config.one_solution_dir);
    let single_results = test_puzzles_one_solution(&single_puzzles);
    export_results_to_csv(&single_results, "single_solution_results.csv").unwrap();

    // Test multiple solution puzzles
    println!("\nTESTING MULTIPLE SOLUTION PUZZLES");
    println!("=================================");
    let multi_puzzles = scan_directory_for_puzzles(&config.all_solutions_dir);
    let multi_results = test_puzzles_all_solutions(&multi_puzzles);
    export_results_to_csv(&multi_results, "multiple_solution_results.csv").unwrap();

    // Compare solver modes
    // println!("\nCOMPARING SOLVER MODES");
    // println!("======================");
    // let comparison_puzzles = scan_directory_for_puzzles(&config.comparison_dir);
    // let comparisons = compare_solver_modes(&comparison_puzzles);
    // export_comparison_to_csv(&comparisons, "solver_mode_comparison.csv").unwrap();

    // Summary
    println!("\nSUMMARY");
    println!("=======");
    println!("Single solution puzzles tested: {}", single_results.len());
    println!("Multiple solution puzzles tested: {}", multi_results.len());
    // println!("Comparison puzzles tested: {}", comparisons.len());

    // Print any puzzles with multiple solutions found in single mode (shouldn't happen for proper Sudoku)
    // let multi_in_single: Vec<_> = single_results.iter()
    //     .filter(|r| r.solution_count > 1)
    //     .collect();
    // if !multi_in_single.is_empty() {
    //     println!("\nWARNING: Found puzzles with multiple solutions in single-solution directory:");
    //     for result in multi_in_single {
    //         println!("  {}: {} solutions", result.filename, result.solution_count);
    //     }
    // }
}

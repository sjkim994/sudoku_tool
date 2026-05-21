use clap::Parser;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

use sudoku_tool::core::solvers::bf_solver::{SolverStats, find_one_solution};
use sudoku_tool::core::sudoku::Sudoku;

/*
    CLI Command format:
        Required arguments: -i for input path and -o for output path

        Optional arguments:
            -l for the maximum number of puzzles to process
            -n for the sample (process every nth puzzle)
            -s for a random seed for sampling
            -p show progress every n puzzles
*/
#[derive(Parser)]
#[command(name = "Sudoku Solver Processor (Simple CSV)")]
#[command(about = "Process Sudoku puzzles from a simple CSV with id and puzzle columns")]
struct Cli {
    /// Input CSV file path (must have columns: id, puzzle)
    #[arg(short, long)]
    input: PathBuf,

    /// Output CSV file path
    #[arg(short, long)]
    output: PathBuf,

    /// Number of puzzles to process (use 0 for all)
    #[arg(short, long, default_value_t = 0)]
    limit: usize,

    /// Sampling rate (process every Nth puzzle)
    #[arg(short = 'n', long, default_value_t = 1)]
    sample: usize,

    /// Random seed for sampling (optional)
    #[arg(short, long)]
    seed: Option<u64>,

    /// Show progress every N puzzles
    #[arg(short, long, default_value_t = 1000)]
    progress: usize,
}

#[derive(Debug, Deserialize)]
struct InputPuzzle {
    id: u32,
    puzzle: String,
}

#[derive(Debug, Serialize)]
struct OutputStats {
    // From InputPuzzle
    id: u32,
    puzzle: String,

    // Solver performance metrics
    solutions_found: usize,
    nodes_explored: usize,
    max_recursion_depth: usize,
    solve_time_ms: u128,
    is_solved: bool,
    leaves: usize,
}

fn process_puzzles(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(&cli.input)?;
    let mut wtr = Writer::from_path(&cli.output)?;

    // Write header
    wtr.serialize(OutputStats {
        id: 0,
        puzzle: "puzzle".to_string(),
        solutions_found: 0,
        nodes_explored: 0,
        max_recursion_depth: 0,
        solve_time_ms: 0,
        is_solved: false,
        leaves: 0,
    })?;

    let mut processed = 0;
    let mut total_time = 0u128;
    let mut total_nodes = 0usize;
    let mut solved_count = 0;

    // Initialize random number generator if seed is provided
    let mut rng = cli.seed.map(|seed| {
        use rand::SeedableRng;
        rand::rngs::StdRng::seed_from_u64(seed)
    });

    for (i, result) in rdr.deserialize().enumerate() {
        // Apply sampling
        if cli.sample > 1 {
            if let Some(ref mut rng) = rng {
                use rand::Rng;
                if !rng.random_ratio(1, cli.sample as u32) {
                    continue;
                }
            } else {
                if i % cli.sample != 0 {
                    continue;
                }
            }
        }

        // Apply limit
        if cli.limit > 0 && processed >= cli.limit {
            break;
        }

        let record: InputPuzzle = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Skipping malformed row {}: {}", i, e);
                continue;
            }
        };
        processed += 1;

        // Progress reporting
        if processed % cli.progress == 0 {
            let avg_time = if processed > 0 {
                total_time / processed as u128
            } else {
                0
            };
            let avg_nodes = if processed > 0 {
                total_nodes / processed
            } else {
                0
            };
            let solved_percent = if processed > 0 {
                (solved_count as f64 / processed as f64) * 100.0
            } else {
                0.0
            };
            println!(
                "Processed {} puzzles (avg: {} ms, {} nodes, solved: {:.1}%)",
                processed, avg_time, avg_nodes, solved_percent
            );
        }

        // Convert string to Sudoku
        let puzzle = match Sudoku::from_string(&record.puzzle) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Skipping malformed puzzle {} (id: {}): {}", i, record.id, e);
                continue;
            }
        };

        // Solve
        let (solution, stats) = find_one_solution(&puzzle);
        total_time += stats.search_duration.as_millis();
        total_nodes += stats.nodes_explored;

        if solution.is_some() {
            solved_count += 1;
        }

        // Write results
        wtr.serialize(OutputStats {
            id: record.id,
            puzzle: record.puzzle,
            solutions_found: stats.solutions_found,
            nodes_explored: stats.nodes_explored,
            max_recursion_depth: stats.max_recursion_depth,
            solve_time_ms: stats.search_duration.as_millis(),
            is_solved: solution.is_some(),
            leaves: stats.leaves,
        })?;

        // Flush periodically to avoid data loss
        if processed % 100 == 0 {
            wtr.flush()?;
        }
    }

    println!("{}", "=".repeat(50));
    println!("COMPLETED! Processed {} puzzles total", processed);
    if processed > 0 {
        println!(
            "Final averages: {} ms/puzzle, {} nodes/puzzle",
            total_time / processed as u128,
            total_nodes / processed
        );
        println!(
            "Success rate: {:.1}% ({}/{} puzzles solved)",
            (solved_count as f64 / processed as f64) * 100.0,
            solved_count,
            processed
        );
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Sudoku Solver Processor (Simple CSV Format)");
    println!("Input: {:?}", cli.input);
    println!("Output: {:?}", cli.output);
    println!(
        "Limit: {}",
        if cli.limit > 0 {
            cli.limit.to_string()
        } else {
            "all".to_string()
        }
    );
    println!(
        "Sampling: {}",
        if cli.sample > 1 {
            format!("1/{}", cli.sample)
        } else {
            "all".to_string()
        }
    );
    if let Some(seed) = cli.seed {
        println!("Random seed: {}", seed);
    }
    println!("Progress reporting: every {} puzzles", cli.progress);
    println!("{}", "=".repeat(50));

    process_puzzles(&cli)
}

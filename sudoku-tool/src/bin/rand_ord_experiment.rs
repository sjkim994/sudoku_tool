use clap::Parser;
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

use sudoku_tool::core::solvers::bf_solver::{
    SolverStats, find_one_solution, find_one_solution_rand_rowcol_order,
};
use sudoku_tool::core::sudoku::Sudoku;

// =============================================================================
// RANDOM ORDERING EXPERIMENT BINARY
//
// This binary runs multiple randomized solver executions on Sudoku puzzles to
// analyze performance variance across different variable ordering heuristics.
//
// USAGE:
//   cargo run --bin rand_ord_experiment -- [FLAGS] [OPTIONS]
//
// REQUIRED FLAGS/OPTIONS:
//   -i, --input <INPUT>      Input CSV file containing puzzle data
//   -o, --output <OUTPUT>    Output CSV file for experiment results
//
// OPTIONAL FLAGS/OPTIONS:
//   -n, --sample-puzzles <SAMPLE_PUZZLES>
//                           Number of puzzles to sample from input [default: 100]
//   -r, --runs-per-puzzle <RUNS_PER_PUZZLE>
//                           Number of random runs per puzzle [default: 1000]
//   -s, --seed <SEED>           Random seed for reproducible sampling
//   -p, --progress <PROGRESS>
//                           Show progress every N puzzles [default: 10]
//   -h, --help              Print help information
//
// EXAMPLE COMMANDS:
//   # Basic run with default parameters
//   cargo run --bin rand_ord_experiment -- -i puzzles.csv -o results.csv
//
//   # Run with custom sampling and reproducible results
//   cargo run --bin rand_ord_experiment -- \
//     -i puzzles.csv -o results.csv \
//     -s 50 -r 500 --seed 12345 -p 5
//
//   # Large-scale experiment
//   cargo run --bin rand_ord_experiment -- \
//     -i large_dataset.csv -o experiment_results.csv \
//     -s 1000 -r 100 --progress 100
// =============================================================================

#[derive(Parser)]
#[command(name = "Random Ordering Experiment")]
#[command(about = "Run solver with random orderings to analyze performance variance")]
struct Cli {
    /// Input CSV file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output CSV file path
    #[arg(short, long)]
    output: PathBuf,

    /// Number of puzzles to sample from input
    #[arg(short = 'n', long, default_value_t = 100)]
    sample_puzzles: usize,

    /// Number of random runs per puzzle
    #[arg(short = 'r', long, default_value_t = 1000)]
    runs_per_puzzle: usize,

    /// Random seed for reproducible sampling
    #[arg(short, long)]
    seed: Option<u64>,

    /// Show progress every N puzzles
    #[arg(short, long, default_value_t = 10)]
    progress: usize,
}

#[derive(Debug, Deserialize)]
struct InputPuzzle {
    id: u32,
    puzzle: String,
    solution: String,
    clues: u8,
    difficulty: f32,
}

#[derive(Debug, Serialize)]
struct RandomRunStats {
    // Puzzle identification
    puzzle_id: u32,
    puzzle: String,
    clues: u8,
    difficulty: f32,

    // Run identification
    run_id: u32,

    // Solver performance metrics
    solutions_found: usize,
    nodes_explored: usize,
    max_recursion_depth: usize,
    solve_time_ms: u128,
    is_solved: bool,
    leaves: usize,
    backtracks: usize,
}

fn run_random_ordering_experiment(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(&cli.input)?;
    let mut wtr = Writer::from_path(&cli.output)?;

    // Write header
    wtr.serialize(RandomRunStats {
        puzzle_id: 0,
        puzzle: "puzzle".to_string(),
        clues: 0,
        difficulty: 0.0,
        run_id: 0,
        solutions_found: 0,
        nodes_explored: 0,
        max_recursion_depth: 0,
        solve_time_ms: 0,
        is_solved: false,
        leaves: 0,
        backtracks: 0,
    })?;

    let mut processed_puzzles = 0;
    let mut total_runs = 0;

    // Initialize random number generator for puzzle sampling
    let mut rng = cli.seed.map(|seed| {
        use rand::SeedableRng;
        rand::rngs::StdRng::seed_from_u64(seed)
    });

    // Collect all puzzles first for sampling
    let all_puzzles: Vec<InputPuzzle> = rdr.deserialize().collect::<Result<_, _>>()?;

    // Sample puzzles (either random or first N)
    let sampled_puzzles: Vec<&InputPuzzle> = if let Some(ref mut rng) = rng {
        use rand::seq::SliceRandom;
        let mut sampled: Vec<&InputPuzzle> = all_puzzles.iter().collect();
        sampled.shuffle(rng);
        sampled.into_iter().take(cli.sample_puzzles).collect()
    } else {
        all_puzzles.iter().take(cli.sample_puzzles).collect()
    };

    println!(
        "Running experiment on {} puzzles, {} runs each",
        sampled_puzzles.len(),
        cli.runs_per_puzzle
    );

    for (puzzle_idx, puzzle_record) in sampled_puzzles.iter().enumerate() {
        processed_puzzles += 1;

        // Progress reporting
        if processed_puzzles % cli.progress == 0 {
            println!(
                "Processing puzzle {}/{}",
                processed_puzzles,
                sampled_puzzles.len()
            );
        }

        // Convert string to Sudoku
        let puzzle = match Sudoku::from_string(&puzzle_record.puzzle) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "Skipping malformed puzzle {} (id: {}): {}",
                    puzzle_idx, puzzle_record.id, e
                );
                continue;
            }
        };

        // ADDED: Run baseline with default ordering (run_id = 0)
        let (baseline_solution, baseline_stats) = find_one_solution(&puzzle);

        wtr.serialize(RandomRunStats {
            puzzle_id: puzzle_record.id,
            puzzle: puzzle_record.puzzle.clone(),
            clues: puzzle_record.clues,
            difficulty: puzzle_record.difficulty,
            run_id: 0, // Baseline run gets ID 0
            solutions_found: baseline_stats.solutions_found,
            nodes_explored: baseline_stats.nodes_explored,
            max_recursion_depth: baseline_stats.max_recursion_depth,
            solve_time_ms: baseline_stats.search_duration.as_millis(),
            is_solved: baseline_solution.is_some(),
            leaves: baseline_stats.leaves,
            backtracks: baseline_stats.backtracks,
        })?;
        total_runs += 1;

        // Run multiple random orderings
        for run in 1..cli.runs_per_puzzle {
            total_runs += 1;

            let (solution, stats) = find_one_solution_rand_rowcol_order(&puzzle);

            // Write results for each run
            wtr.serialize(RandomRunStats {
                puzzle_id: puzzle_record.id,
                puzzle: puzzle_record.puzzle.clone(),
                clues: puzzle_record.clues,
                difficulty: puzzle_record.difficulty,
                run_id: run as u32,
                solutions_found: stats.solutions_found,
                nodes_explored: stats.nodes_explored,
                max_recursion_depth: stats.max_recursion_depth,
                solve_time_ms: stats.search_duration.as_millis(),
                is_solved: solution.is_some(),
                leaves: stats.leaves,
                backtracks: stats.backtracks,
            })?;

            // Flush periodically to avoid data loss
            if total_runs % 100 == 0 {
                wtr.flush()?;
            }
        }
    }

    println!(
        "Completed! Processed {} puzzles, {} total runs",
        processed_puzzles, total_runs
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Random Ordering Experiment");
    println!("Input: {:?}", cli.input);
    println!("Output: {:?}", cli.output);
    println!("Puzzles to sample: {}", cli.sample_puzzles);
    println!("Runs per puzzle: {}", cli.runs_per_puzzle);
    if let Some(seed) = cli.seed {
        println!("Random seed: {}", seed);
    }
    println!("Progress reporting: every {} puzzles", cli.progress);
    println!("{}", "=".repeat(50));

    run_random_ordering_experiment(&cli)
}

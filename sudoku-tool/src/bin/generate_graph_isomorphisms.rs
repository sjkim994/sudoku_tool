use clap::Parser;
use csv::Writer;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;

use sudoku_tool::core::sudoku::Sudoku;
use sudoku_tool::core::transformers::*;

// =============================================================================
// GENERATE RANDOM SUDOKU ISOMORPHISMS
//
// This binary takes a single Sudoku puzzle and generates random isomorphic
// puzzles through geometric transformations and digit relabeling.
//
// USAGE:
//   cargo run --bin generate_graph_isomorphisms -- [OPTIONS]
//
// REQUIRED:
//   -i, --input <INPUT>      Input text file containing a single Sudoku puzzle string
//   -o, --output <OUTPUT>    Output CSV file for isomorphic puzzles
//   -n, --count <COUNT>      Number of isomorphic puzzles to generate
//
// OPTIONAL:
//   -s, --seed <SEED>        Random seed for reproducible sampling
//   -v, --verbose            Print progress and statistics
//   -h, --help              Print help information
//
// EXAMPLE COMMANDS:
//   # Generate 1000 random isomorphisms
//   cargo run --bin generate_graph_isomorphisms -- -i puzzle.txt -o isomorphisms.csv -n 1000
//
//   # Generate 100 random isomorphisms with verbose output
//   cargo run --bin generate_graph_isomorphisms -- -i puzzle.txt -o isomorphisms.csv -n 100 -v
//
//   # Generate reproducible sample for research
//   cargo run --bin generate_graph_isomorphisms -- -i puzzle.txt -o isomorphisms.csv -n 500 -s 42 -v
// =============================================================================

#[derive(Parser)]
#[command(name = "Generate Random Sudoku Isomorphisms")]
#[command(
    about = "Generate random isomorphic Sudoku puzzles through geometric transformations and digit relabeling"
)]
struct Cli {
    /// Input text file containing a single Sudoku puzzle string
    #[arg(short, long)]
    input: PathBuf,

    /// Output CSV file path
    #[arg(short, long)]
    output: PathBuf,

    /// Number of isomorphic puzzles to generate
    #[arg(short, long)]
    ncount: usize,

    /// Random seed for reproducible sampling
    #[arg(short, long)]
    seed: Option<u64>,

    /// Print progress and statistics
    #[arg(short, long)]
    verbose: bool,
}

fn read_puzzle_from_file(path: &PathBuf) -> Result<Sudoku, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let cleaned: String = content.chars().filter(|c| !c.is_whitespace()).collect();

    if cleaned.is_empty() {
        return Err("Input file is empty".into());
    }

    Sudoku::from_string(&cleaned).map_err(|e| e.into())
}

// Get all 8 rotations and reflections
fn get_dihedral_transformations() -> Vec<fn(&Sudoku) -> Sudoku> {
    vec![
        |s| s.clone(),               // identity
        rotations::rotate_90,        // 90° rotation
        rotations::rotate_180,       // 180° rotation
        rotations::rotate_270,       // 270° rotation
        reflections::h_reflect,      // horizontal reflection
        reflections::v_reflect,      // vertical reflection
        reflections::d_reflect,      // main diagonal reflection
        reflections::dprime_reflect, // anti-diagonal reflection
    ]
}

// Get all 6 band/stack/row/col permutations as sequences of transpositions
fn get_permutations() -> Vec<Vec<(usize, usize)>> {
    vec![
        vec![],               // identity
        vec![(1, 2)],         // swap 1 and 2
        vec![(0, 1)],         // swap 0 and 1
        vec![(0, 2), (0, 1)], // cycle: 0->1, 1->2, 2->0
        vec![(0, 1), (0, 2)], // cycle: 0->2, 2->1, 1->0
        vec![(0, 2)],         // swap 0 and 2
    ]
}

// Generate a random digit relabeling (random permutation of digits 1-9)
fn random_relabeling<R: Rng>(rng: &mut R) -> Vec<(usize, usize)> {
    let mut digits: Vec<usize> = (1..=9).collect();
    digits.shuffle(rng);

    // Convert permutation to sequence of transpositions
    let mut transpositions = Vec::new();
    let mut current: Vec<usize> = (1..=9).collect();

    for i in 0..9 {
        if current[i] != digits[i] {
            let pos = current.iter().position(|&x| x == digits[i]).unwrap();
            for j in (i + 1..=pos).rev() {
                transpositions.push((current[j - 1], current[j]));
                current.swap(j - 1, j);
            }
        }
    }

    transpositions.dedup();
    transpositions
}

// Apply a sequence of transpositions for band/stack/row/col permutations
fn apply_permutation_sequence<F>(
    sudoku: &Sudoku,
    transpositions: &[(usize, usize)],
    permute_fn: F,
) -> Sudoku
where
    F: Fn(&Sudoku, (usize, usize)) -> Sudoku,
{
    let mut result = sudoku.clone();
    for &(a, b) in transpositions {
        result = permute_fn(&result, (a, b));
    }
    result
}

// Generate a random isomorphism (always includes digit relabeling)
fn random_isomorphism<R: Rng>(original: &Sudoku, rng: &mut R) -> Sudoku {
    let dihedral_transforms = get_dihedral_transformations();
    let permutations = get_permutations();

    // Randomly select each transformation
    let dihedral = dihedral_transforms.choose(rng).unwrap();
    let band_perm = permutations.choose(rng).unwrap();
    let stack_perm = permutations.choose(rng).unwrap();
    let row_perms = [
        permutations.choose(rng).unwrap(),
        permutations.choose(rng).unwrap(),
        permutations.choose(rng).unwrap(),
    ];
    let col_perms = [
        permutations.choose(rng).unwrap(),
        permutations.choose(rng).unwrap(),
        permutations.choose(rng).unwrap(),
    ];

    // Apply geometric transformations in sequence
    let mut result = dihedral(original);
    result = apply_permutation_sequence(&result, band_perm, stacks_bands::permute_bands);
    result = apply_permutation_sequence(&result, stack_perm, stacks_bands::permute_stacks);

    for band in 0..3 {
        result = apply_permutation_sequence(&result, row_perms[band], |s, p| {
            stacks_bands::permute_rows_in_band(s, band, p)
        });
    }

    for stack in 0..3 {
        result = apply_permutation_sequence(&result, col_perms[stack], |s, p| {
            stacks_bands::permute_cols_in_stack(s, stack, p)
        });
    }

    // Always apply digit relabeling
    let relabel = random_relabeling(rng);
    result = apply_permutation_sequence(&result, &relabel, relabeling::relabel);

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Generate Random Sudoku Isomorphisms");
    println!("Input: {:?}", cli.input);
    println!("Output: {:?}", cli.output);
    println!("Generating {} random isomorphisms", cli.ncount);
    if let Some(seed) = cli.seed {
        println!("Random seed: {}", seed);
    }
    println!("{}", "=".repeat(50));

    // Read input puzzle
    println!("Reading puzzle from file...");
    let puzzle = match read_puzzle_from_file(&cli.input) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading puzzle: {}", e);
            return Err(e);
        }
    };

    println!("Original puzzle: {}", puzzle.to_string_rep());
    println!();

    // Generate random isomorphisms
    println!("Generating {} random isomorphisms...", cli.ncount);
    let start_time = Instant::now();

    let mut rng: Box<dyn rand::RngCore> = match cli.seed {
        Some(seed) => Box::new(rand::rngs::StdRng::seed_from_u64(seed)),
        None => Box::new(rand::rngs::ThreadRng::default()),
    };

    let mut isomorphisms = HashSet::new();
    let mut attempts = 0;
    let max_attempts = cli.ncount * 10; // Prevent infinite loops

    while isomorphisms.len() < cli.ncount && attempts < max_attempts {
        let isomorphism = random_isomorphism(&puzzle, &mut rng);
        isomorphisms.insert(isomorphism.to_string_rep());
        attempts += 1;

        if cli.verbose && isomorphisms.len() % 100 == 0 && isomorphisms.len() > 0 {
            println!(
                "  Generated {} unique isomorphisms ({} attempts)",
                isomorphisms.len(),
                attempts
            );
        }
    }

    if isomorphisms.len() < cli.ncount {
        println!(
            "Warning: Only found {} unique isomorphisms out of {} requested (space may be smaller due to puzzle symmetry)",
            isomorphisms.len(),
            cli.ncount
        );
    }

    let elapsed = start_time.elapsed();
    println!();
    println!("Sampling complete!");
    println!("  Unique isomorphisms generated: {}", isomorphisms.len());
    println!("  Total attempts: {}", attempts);
    println!("  Time elapsed: {:.2?}", elapsed);
    println!(
        "  Average rate: {:.0} isomorphisms/sec",
        isomorphisms.len() as f64 / elapsed.as_secs_f64()
    );

    // Write to CSV
    println!();
    println!("Writing results to CSV...");
    let mut wtr = Writer::from_path(&cli.output)?;
    wtr.write_record(&["id", "puzzle"])?;

    for (id, puzzle_str) in isomorphisms.iter().enumerate() {
        wtr.write_record(&[id.to_string(), puzzle_str.clone()])?;

        if cli.verbose && (id + 1) % 1000 == 0 {
            println!("  Written {} isomorphisms", id + 1);
        }
    }

    wtr.flush()?;

    println!();
    println!("Completed!");
    println!("Output written to: {:?}", cli.output);

    Ok(())
}

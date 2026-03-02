use crate::core::sudoku::Sudoku;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RelabelError {
    #[error("Digits must be between 1 and 9, got {0} and {1}")]
    InvalidDigit(u8, u8),
    #[error("Cannot swap digit {0} with itself")]
    SameDigit(u8),
    #[error("Invalid permutation: {0}")]
    InvalidPermutation(String),
}

/// Relabel digits in a Sudoku puzzle according to a permutation
/// permutation[old_digit - 1] = new_digit
/// e.g., [9, 8, 7, 6, 5, 4, 3, 2, 1] would reverse all digits (1↔9, 2↔8, 3↔7, etc.)
pub fn relabel(sudoku: &Sudoku, permutation: &[u8; 9]) -> Result<Sudoku, RelabelError> {
    // Validate permutation
    let mut seen = HashSet::new();
    for &digit in permutation {
        if digit < 1 || digit > 9 {
            return Err(RelabelError::InvalidPermutation(
                format!("Invalid digit {} in permutation. Digits must be 1-9.", digit)
            ));
        }
        if !seen.insert(digit) {
            return Err(RelabelError::InvalidPermutation(
                format!("Duplicate digit {} in permutation. Permutation must be bijective.", digit)
            ));
        }
    }
    if seen.len() != 9 {
        return Err(RelabelError::InvalidPermutation(
            "Permutation must contain exactly 9 unique digits 1-9.".to_string()
        ));
    }
    
    let mut new_sudoku = Sudoku::new();
    
    for row in 0..9 {
        for col in 0..9 {
            if let Some(old_value) = sudoku.get_solved_value(row, col) {
                // Map old value to new value using permutation
                let new_value = permutation[(old_value - 1) as usize];
                new_sudoku.set_cell(row, col, new_value)
                    .map_err(|e| RelabelError::InvalidPermutation(
                        format!("Failed to set cell during relabeling: {}", e)
                    ))?;
            }
        }
    }
    
    Ok(new_sudoku)
}

/// Core function: Try to swap two digits, returns Result
pub fn try_swap_digits(sudoku: &Sudoku, digit1: u8, digit2: u8) -> Result<Sudoku, RelabelError> {
    // Validate digits
    if digit1 < 1 || digit1 > 9 || digit2 < 1 || digit2 > 9 {
        return Err(RelabelError::InvalidDigit(digit1, digit2));
    }
    
    if digit1 == digit2 {
        return Err(RelabelError::SameDigit(digit1));
    }
    
    // Create permutation that swaps digit1 and digit2
    let mut permutation = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    permutation[(digit1 - 1) as usize] = digit2;
    permutation[(digit2 - 1) as usize] = digit1;
    
    Ok(relabel(sudoku, &permutation)?)
}

/// Experiment-friendly version: Logs warnings and returns original on error
pub fn swap_digits_experiment(sudoku: &Sudoku, digit1: u8, digit2: u8) -> Sudoku {
    match try_swap_digits(sudoku, digit1, digit2) {
        Ok(result) => result,
        Err(RelabelError::SameDigit(d)) => {
            // In experiments, we might generate random digits that could be the same
            // Log a warning and return the original puzzle
            eprintln!("[EXPERIMENT] Warning: Skipping self-swap of digit {}", d);
            sudoku.clone()
        }
        Err(RelabelError::InvalidDigit(d1, d2)) => {
            // This should rarely happen if we generate digits correctly
            // Log an error and return the original puzzle
            eprintln!("[EXPERIMENT] Error: Invalid digits {} and {}. Returning original.", d1, d2);
            sudoku.clone()
        }
        Err(RelabelError::InvalidPermutation(msg)) => {
            // This is a programming error, but we don't want experiments to crash
            eprintln!("[EXPERIMENT] Critical: Invalid permutation: {}. Returning original.", msg);
            sudoku.clone()
        }
    }
}

/// Development version: Panics on error (for tests and development)
pub fn swap_digits(sudoku: &Sudoku, digit1: u8, digit2: u8) -> Sudoku {
    try_swap_digits(sudoku, digit1, digit2)
        .unwrap_or_else(|e| panic!("swap_digits failed: {}", e))
}

// ========== RANDOM RELABELING ==========

/// Core version: Returns Result
pub fn try_random_relabel<R: Rng>(sudoku: &Sudoku, rng: &mut R) -> Result<Sudoku, RelabelError> {
    let permutation = random_permutation(rng);
    relabel(sudoku, &permutation)
}

/// Experiment version: Never crashes
pub fn random_relabel_experiment<R: Rng>(sudoku: &Sudoku, rng: &mut R) -> Sudoku {
    match try_random_relabel(sudoku, rng) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("[EXPERIMENT] Error in random_relabel: {}. Returning original.", e);
            sudoku.clone()
        }
    }
}

/// Development version: Panics on error
pub fn random_relabel<R: Rng>(sudoku: &Sudoku, rng: &mut R) -> Sudoku {
    try_random_relabel(sudoku, rng)
        .unwrap_or_else(|e| panic!("random_relabel failed: {}", e))
}

// ========== SAMPLE RELABELINGS ==========

/// Experiment version
pub fn sample_relabelings_experiment(sudoku: &Sudoku, sample_size: usize) -> Vec<Sudoku> {
    let mut rng = rand::rng();
    let mut puzzles = Vec::with_capacity(sample_size);
    let mut seen_permutations = HashSet::new();

    // Always include identity permutation (no change)
    puzzles.push(sudoku.clone());
    seen_permutations.insert([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    while puzzles.len() < sample_size {
        let permutation = random_permutation(&mut rng);

        if seen_permutations.insert(permutation) {
            match relabel(sudoku, &permutation) {
                Ok(puzzle) => puzzles.push(puzzle),
                Err(e) => {
                    eprintln!("[EXPERIMENT] Error in sample_relabelings: {}. Skipping permutation.", e);
                }
            }
        }
    }

    puzzles
}

/// Development version
pub fn sample_relabelings(sudoku: &Sudoku, sample_size: usize) -> Vec<Sudoku> {
    let mut rng = rand::rng();
    let mut puzzles = Vec::with_capacity(sample_size);
    let mut seen_permutations = HashSet::new();

    // Always include identity permutation (no change)
    puzzles.push(sudoku.clone());
    seen_permutations.insert([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    while puzzles.len() < sample_size {
        let permutation = random_permutation(&mut rng);

        if seen_permutations.insert(permutation) {
            puzzles.push(
                relabel(sudoku, &permutation)
                    .unwrap_or_else(|e| panic!("sample_relabelings failed: {}", e))
            );
        }
    }

    puzzles
}

// ========== ALL RELABELINGS ==========

/// Helper function for all_relabelings (returns Result)
fn try_generate_permutations(
    digits: &mut [u8],  // Change to mutable slice instead of Vec
    k: usize,
    sudoku: &Sudoku,
    puzzles: &mut Vec<Sudoku>,
    errors: &mut Vec<RelabelError>,
) {
    if k == 1 {
        // FIX: Create array directly from slice
        let mut permutation = [0u8; 9];
        permutation.copy_from_slice(digits);
        
        match relabel(sudoku, &permutation) {
            Ok(puzzle) => puzzles.push(puzzle),
            Err(e) => errors.push(e),
        }
    } else {
        try_generate_permutations(digits, k - 1, sudoku, puzzles, errors);

        for i in 0..(k - 1) {
            if k % 2 == 0 {
                digits.swap(i, k - 1);
            } else {
                digits.swap(0, k - 1);
            }
            try_generate_permutations(digits, k - 1, sudoku, puzzles, errors);
        }
    }
}

/// Experiment version: Logs errors but continues
pub fn all_relabelings_experiment(sudoku: &Sudoku) -> (Vec<Sudoku>, Vec<RelabelError>) {
    let mut puzzles = Vec::new();
    let mut errors = Vec::new();
    let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];  // Array instead of Vec

    try_generate_permutations(&mut digits, 9, sudoku, &mut puzzles, &mut errors);
    
    if !errors.is_empty() {
        eprintln!("[EXPERIMENT] Warning: {} permutations failed in all_relabelings", errors.len());
    }
    
    (puzzles, errors)
}

/// Development version: Panics on first error
pub fn all_relabelings(sudoku: &Sudoku) -> Vec<Sudoku> {
    let mut puzzles = Vec::new();
    let mut digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];  // Array instead of Vec
    let mut errors = Vec::new();

    try_generate_permutations(&mut digits, 9, sudoku, &mut puzzles, &mut errors);
    
    if !errors.is_empty() {
        panic!("all_relabelings failed: {:?}", errors[0]);
    }
    
    puzzles
}

// ========== ALL PAIRWISE SWAPS ==========

/// Experiment version
pub fn all_pairwise_swaps_experiment(sudoku: &Sudoku) -> Vec<Sudoku> {
    let mut swaps = Vec::new();

    for d1 in 1..=9 {
        for d2 in (d1 + 1)..=9 {
            swaps.push(swap_digits_experiment(sudoku, d1, d2));
        }
    }

    swaps
}

/// Development version
pub fn all_pairwise_swaps(sudoku: &Sudoku) -> Vec<Sudoku> {
    let mut swaps = Vec::new();

    for d1 in 1..=9 {
        for d2 in (d1 + 1)..=9 {
            swaps.push(swap_digits(sudoku, d1, d2));
        }
    }

    swaps
}

// ========== SAMPLE PAIRWISE SWAPS ==========

/// Experiment version
pub fn sample_pairwise_swaps_experiment<R: Rng>(
    sudoku: &Sudoku,
    rng: &mut R,
    sample_size: usize,
) -> Vec<Sudoku> {
    // Generate all possible pairs
    let mut all_pairs = Vec::new();
    for d1 in 1..=9 {
        for d2 in (d1 + 1)..=9 {
            all_pairs.push((d1, d2));
        }
    }

    // Shuffle and take sample
    all_pairs.shuffle(rng);

    all_pairs
        .iter()
        .take(sample_size.min(all_pairs.len()))
        .map(|&(d1, d2)| swap_digits_experiment(sudoku, d1, d2))
        .collect()
}

/// Development version
pub fn sample_pairwise_swaps<R: Rng>(
    sudoku: &Sudoku,
    rng: &mut R,
    sample_size: usize,
) -> Vec<Sudoku> {
    // Generate all possible pairs
    let mut all_pairs = Vec::new();
    for d1 in 1..=9 {
        for d2 in (d1 + 1)..=9 {
            all_pairs.push((d1, d2));
        }
    }

    // Shuffle and take sample
    all_pairs.shuffle(rng);

    all_pairs
        .iter()
        .take(sample_size.min(all_pairs.len()))
        .map(|&(d1, d2)| swap_digits(sudoku, d1, d2))
        .collect()
}

// ========== UTILITY FUNCTIONS ==========

/// Generate a random digit permutation
pub fn random_permutation<R: Rng>(rng: &mut R) -> [u8; 9] {
    let mut permutation = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    permutation.shuffle(rng);
    permutation
}
use crate::core::sudoku::Sudoku;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::{Duration, Instant};

/*

Algorithm steps:

1. Fill in forced numbers from highest frequency to lowest frequency of hints
2. Markup puzzle
3. Search iteratively for preemptive sets in all rows, cols, and sub-grids
4. Once you locate one, perform crossout action
5. Search for other preemptive sets in the same range (prev hidden)
6. Repeat 4 and 5 until exhausted
7. Continue with iterative search until solution is found or you need to make a random choice.
8. If solution is not found, make a random choice
9. Backtrack where needed

*/

#[derive(Debug, Clone)]
pub struct PencilPaperStats {
    pub solutions_found: usize,
    pub search_duration: Duration,
    pub preemptive_sets_found: usize,
    pub forced_numbers_placed: usize,
    pub iterations: usize,
}

impl Default for PencilPaperStats {
    fn default() -> Self {
        Self {
            solutions_found: 0,
            search_duration: Duration::default(),
            preemptive_sets_found: 0,
            forced_numbers_placed: 0,
            iterations: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreemptiveSet {
    numbers: BTreeSet<u8>,
    cells: Vec<(usize, usize)>,
    // Multiple ranges can apply for sets of size 2-3
    ranges: Vec<RangeType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RangeType {
    Row(usize),
    Column(usize),
    Box(usize), // 0-8: box 0=(0,0), 1=(0,1), 2=(0,2), 3=(1,0), etc.
}

impl PreemptiveSet {
    fn new(numbers: BTreeSet<u8>, cells: Vec<(usize, usize)>) -> Self {
        let mut ranges = Vec::new();
        
        // Determine which ranges this set applies to
        if let Some(row) = Self::get_common_row(&cells) {
            ranges.push(RangeType::Row(row));
        }
        
        if let Some(col) = Self::get_common_column(&cells) {
            ranges.push(RangeType::Column(col));
        }
        
        if let Some(box_idx) = Self::get_common_box(&cells) {
            ranges.push(RangeType::Box(box_idx));
        }
        
        PreemptiveSet { numbers, cells, ranges }
    }
    
    fn get_common_row(cells: &[(usize, usize)]) -> Option<usize> {
        let first_row = cells[0].0;
        if cells.iter().all(|&(r, _)| r == first_row) {
            Some(first_row)
        } else {
            None
        }
    }
    
    fn get_common_column(cells: &[(usize, usize)]) -> Option<usize> {
        let first_col = cells[0].1;
        if cells.iter().all(|&(_, c)| c == first_col) {
            Some(first_col)
        } else {
            None
        }
    }
    
    fn get_common_box(cells: &[(usize, usize)]) -> Option<usize> {
        let first_box = (cells[0].0 / 3) * 3 + (cells[0].1 / 3);
        if cells.iter().all(|&(r, c)| (r / 3) * 3 + (c / 3) == first_box) {
            Some(first_box)
        } else {
            None
        }
    }
    
    // Check if this preemptive set applies to a specific range
    fn applies_to_range(&self, range_type: &RangeType) -> bool {
        self.ranges.contains(range_type)
    }
}


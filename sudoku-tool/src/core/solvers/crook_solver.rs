use crate::core::sudoku::Sudoku;
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
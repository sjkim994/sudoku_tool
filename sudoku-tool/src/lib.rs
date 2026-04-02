pub mod core {
    pub mod solvers {
        pub mod bf_solver;
        // Add other solver modules as they become public
    }
    pub mod sudoku;
    pub mod transformers {
        pub mod reflections;
        pub mod relabeling;
        pub mod rotations;
        pub mod stacks_bands;
    }
    // Add other core modules as needed
}

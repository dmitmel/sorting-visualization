use super::{Algorithm, Array};

pub struct All;

impl Algorithm for All {
    fn sort(&self, array: Array) {
        // Get the list of algorithms
        loop {
            for (_name, algo) in super::all() {
                // Run the algorithm on the array
                algo.sort(array.clone());
                // Shuffles the array
                array.shuffle();
            }
        }
    }

    fn name(&self) -> String {
        "All algorithms".to_string()
    }
}

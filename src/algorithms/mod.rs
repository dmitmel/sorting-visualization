//! Different sorting algorithms as well as the general
//! [`Algorithm`](crate::algorithms::Algorithm) trait.

use std::collections::HashMap;

// re-export Array struct for use in algorithms
pub use crate::array::Array;

/// The general trait for all sorting algorithms.
pub trait Algorithm {
  /// Sorts a given [array](crate::array::Array). This method is called in a so
  /// called "algorithm thread".
  fn sort(&self, array: Array);
  /// Returns the name of the algorithm that will be displayed to the user.
  /// Returned value is an owned [String] so it can be generated at runtime.
  fn name(&self) -> String;
}

/// A shorthand macro that declares modules of all algorithms, re-exports those
/// algorithms and generates a function that returns all algorithms.
macro_rules! algorithms {
  ($($id:ident => $type:ident,)+) => {
    $(
      pub mod $id;
      pub use self::$id::$type;
    )*

    /// Returns a [hashmap](HashMap) of all algorithms. It is used in the
    /// [cli](crate::cli) module.
    pub fn all() -> HashMap<String, Box<dyn Algorithm + Send>> {
      let mut algorithms: HashMap<String, Box<dyn Algorithm + Send>> =
        HashMap::new();
      $(algorithms.insert(stringify!($id).to_string(), Box::new(self::$id::$type));)*
      algorithms
    }
  };
}

algorithms![
  bubble => BubbleSort,
  cocktail => CocktailSort,
  cycle => CycleSort,
  gnome => GnomeSort,
  heap => HeapSort,
  insertion => InsertionSort,
  merge => MergeSort,
  quicksort => Quicksort,
  selection => SelectionSort,
  shellsort => Shellsort,
];

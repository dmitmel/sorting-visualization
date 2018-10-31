//! Different sorting algorithms as well as the general
//! [`Algorithm`](::algorithms::Algorithm) trait.

use array::Array;

/// The general trait for all sorting algorithms.
pub trait Algorithm {
  /// Sorts a given [array](::array::Array). This method is called in a so
  /// called "algorithm thread".
  fn sort(&self, array: Array);
  /// Returns the name of the algorithm that will be displayed to the user.
  /// Returned value is an owned [String] so it can be generated at runtime.
  fn name(&self) -> String;
}

/// A shorthand macro for declaring some modules and then re-exporting al
/// contents of those modules.
macro_rules! reexport {
  ($($name:ident),*) => ($(
    pub mod $name;
    pub use self::$name::*;
  )*);
}

reexport![bubble, cycle, gnome, insertion, quicksort, selection, shell];

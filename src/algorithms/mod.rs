//! Different sorting algorithms as well as the general [`Algorithm`] trait.

use array::Array;

/// The general trait for all sorting algorithms.
pub trait Algorithm {
  /// Sorts a given [array](::array::Array). This method is called in a so
  /// called "algorithm thread".
  fn sort(&self, array: Array);
  /// Returns the name of the algorithm that will be displayed to the user.
  fn name(&self) -> &'static str;
}

/// A macro for declaring some modules and then re-exporting all contents of
/// those modules.
macro_rules! reexport {
  ($($name:ident),*) => ($(
    pub mod $name;
    pub use self::$name::*;
  )*);
}

reexport![bubble, cycle, gnome, insertion, quicksort, selection];

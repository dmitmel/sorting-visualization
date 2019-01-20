//! [Cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)

use super::{Algorithm, Array};

/// [Cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)
pub struct CocktailSort;

impl Algorithm for CocktailSort {
    fn sort(&self, array: Array) {}

    fn name(&self) -> String {
      "Cocktail sort".to_string()
    }
}

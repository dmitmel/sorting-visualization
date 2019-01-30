//! [Cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)

use super::{Algorithm, Array};

/// [Cocktail sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)
pub struct CocktailSort;

impl Algorithm for CocktailSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    let mut first = 0;
    let mut last = len - 1;
    let mut swapped: bool = true;

    while swapped {
      swapped = false;
      array.set_color(last, [0.0, 1.0, 0.0, 0.8]);

      for i in first..last {
        array.set_color(i, [0.0, 1.0, 0.0, 0.8]);
        if array.get(i) > array.get(i + 1) {
          array.swap(i, i + 1);
          swapped = true;
        }
        array.wait(5);
        array.reset_color(i);
      }

      array.reset_color(last);

      if !swapped {
        break;
      }

      swapped = false;
      last -= 1;
      array.set_color(first, [0.0, 1.0, 0.0, 0.8]);

      for j in (first..last).rev() {
        array.set_color(j, [0.0, 1.0, 0.0, 0.8]);
        if array.get(j) > array.get(j + 1) {
          array.swap(j, j + 1);
          swapped = true;
        }
        array.wait(5);
        array.reset_color(j);
      }

      array.reset_color(first);
      first += 1;
    }
  }

  fn name(&self) -> String {
    "Cocktail sort".to_string()
  }
}

//! [Quicksort](https://en.wikipedia.org/wiki/Quicksort)

use super::{Algorithm, Array};

/// [Quicksort](https://en.wikipedia.org/wiki/Quicksort)
pub struct Quicksort;

impl Algorithm for Quicksort {
  fn sort(&self, array: Array) {
    self.sort_slice(&array, 0, array.len() as isize - 1);
  }

  fn name(&self) -> String {
    "Quicksort".to_string()
  }
}

impl Quicksort {
  #[allow(clippy::range_minus_one)]
  fn sort_slice(&self, array: &Array, low: isize, high: isize) {
    if low < high {
      let pivot = self.partition(array, low, high);

      for i in low..=pivot - 1 {
        array.set_color(i as usize, [0.0, 1.0, 0.0, 0.3]);
      }
      array.set_color(pivot as usize, [1.0, 0.0, 0.0, 1.0]);
      for i in pivot + 1..=high {
        array.set_color(i as usize, [0.0, 0.0, 1.0, 0.3]);
      }

      self.sort_slice(array, low, pivot - 1);
      self.sort_slice(array, pivot + 1, high);

      for i in low..=pivot - 1 {
        array.reset_color(i as usize);
      }
      array.reset_color(pivot as usize);
      for i in pivot + 1..=high {
        array.reset_color(i as usize);
      }
    }
  }

  fn partition(&self, array: &Array, low: isize, high: isize) -> isize {
    let pivot = array.get(high as usize);
    let mut i = low;
    for j in low..high {
      if array.get(j as usize) <= pivot {
        array.swap(i as usize, j as usize);
        i += 1;
      }
      array.wait(15);
    }
    array.swap(i as usize, high as usize);
    i
  }
}

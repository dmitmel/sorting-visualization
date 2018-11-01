//! [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort)

use super::{Algorithm, Array};

/// [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort)
pub struct BubbleSort;

impl Algorithm for BubbleSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 0..len - 1 {
      let last = len - i - 1;
      array.set_color(last, [0.0, 1.0, 0.0, 0.8]);

      for j in 0..last {
        array.set_color(j, [0.0, 1.0, 0.0, 0.8]);
        if array.get(j) > array.get(j + 1) {
          array.swap(j, j + 1);
        }
        array.wait(5);
        array.reset_color(j);
      }

      array.reset_color(last);
    }
  }

  fn name(&self) -> String {
    "Bubble sort".to_string()
  }
}

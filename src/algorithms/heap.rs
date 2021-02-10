//! [Heap sort](https://en.wikipedia.org/wiki/Heapsort)

use super::{Algorithm, Array};

/// [Heap sort](https://en.wikipedia.org/wiki/Heapsort)
pub struct HeapSort;

impl Algorithm for HeapSort {
  fn sort(&self, array: Array) {
    let first = 0;
    let n = array.len();

    for i in (first..n / 2 - 1).rev() {
      heapify(&array, n, i);
    }

    for i in (first..n).rev() {
      array.swap(first, i);
      heapify(&array, i, first);
    }

    fn heapify(array: &Array, n: usize, i: usize) {
      let mut largest = i;
      let left = 2 * i + 1;
      let right = 2 * i + 2;

      if left < n && array.get(left) > array.get(largest) {
        largest = left;
      }

      if right < n && array.get(right) > array.get(largest) {
        largest = right;
      }

      if largest != i {
        array.set_color(i, [0.0, 1.0, 0.0, 0.8]);
        array.swap(i, largest);
        heapify(array, n, largest);
        array.wait(5);
        array.reset_color(i);
      }
    }
  }

  fn name(&self) -> String {
    "HeapSort sort".to_string()
  }
}

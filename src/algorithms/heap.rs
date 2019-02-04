//! [Heap sort](https://en.wikipedia.org/wiki/Heapsort)

  use super::{Algorithm, Array};

  /// [Heap sort](https://en.wikipedia.org/wiki/Heapsort
 pub struct HeapSort;

  impl Algorithm for HeapSort {
     fn sort(&self, array: Array) {
         let n = array.len();

         for i in ((n / 2 - 1)..0).rev() {
             heapify(&array, n, i);
         }

         for i in ((n - 1)..0).rev() {
             array.swap(0, i);
             heapify(&array, i, 0);
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

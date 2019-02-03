//! [Heap sort](https://en.wikipedia.org/wiki/Heapsort)

  use super::{Algorithm, Array};

  /// [Heap sort](https://en.wikipedia.org/wiki/Heapsort
 pub struct HeapSort;

  impl Algorithm for HeapSort {
     fn sort(&self, array: Array) {}

      fn name(&self) -> String {
       "HeapSort sort".to_string()
     }
 }

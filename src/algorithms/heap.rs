//! [Heap sort](https://en.wikipedia.org/wiki/Heapsort)

  use super::{Algorithm, Array};

  /// [Heap sort](https://en.wikipedia.org/wiki/Heapsort
 pub struct HeapSort;

  impl Algorithm for HeapSort {
     fn sort(&self, array: Array) {
         let len = array.len();

         for i in (len / 2 - 1..0).rev() {
             heapify(&array, len, i);
         }

         for i in (len - 1..0).rev() {
             let temp = array.get(0);
             array.set(0, i as u32);
             array.set(i, temp);

             heapify(&array, i, 0);
         }

         fn heapify(array: &Array, len: usize, i: usize) {}
     }

      fn name(&self) -> String {
       "HeapSort sort".to_string()
     }
 }

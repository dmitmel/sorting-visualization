use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct BubbleSort;

impl Algorithm for BubbleSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 0..len - 1 {
      for j in 0..len - i - 1 {
        if array.get(j) > array.get(j + 1) {
          array.swap(j, j + 1);
          delay(5);
        }
      }
    }
  }

  fn name(&self) -> &'static str {
    "Bubble Sort"
  }
}

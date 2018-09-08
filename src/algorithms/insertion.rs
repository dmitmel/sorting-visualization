use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct InsertionSort;

impl Algorithm for InsertionSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 1..len {
      let mut j = i;
      while j > 0 && array.get(j - 1) > array.get(j) {
        array.swap(j, j - 1);
        delay(10);
        j -= 1;
      }
    }
  }
}

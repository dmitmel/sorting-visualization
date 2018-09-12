use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct InsertionSort;

impl Algorithm for InsertionSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 1..len {
      array.set_highlighted_index(i);
      let mut j = i;
      while j > 0 && array.get(j - 1) > array.get(j) {
        array.swap(j, j - 1);
        delay(5);
        j -= 1;
      }
    }
  }

  fn name(&self) -> &'static str {
    "Insertion Sort"
  }
}

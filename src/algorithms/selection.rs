use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct SelectionSort;

impl Algorithm for SelectionSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 0..len - 1 {
      let mut min_i = i;
      for j in i + 1..len {
        if array.get(j) < array.get(min_i) {
          min_i = j;
        }
        delay(2);
      }

      array.swap(i, min_i);
      delay(20);
    }
  }

  fn name(&self) -> &'static str {
    "Selection Sort"
  }
}

use algorithms::Algorithm;
use array::Array;

pub struct InsertionSort;

impl Algorithm for InsertionSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 1..len {
      array.set_color(i, [0.0, 1.0, 0.0, 0.8]);
      array.wait(5);

      let mut j = i;
      while j > 0 && array.get(j - 1) > array.get(j) {
        array.swap(j, j - 1);
        j -= 1;

        array.set_color(j, [0.0, 1.0, 0.0, 0.8]);
        array.wait(5);
        array.reset_color(j);
      }

      array.reset_color(i);
    }
  }

  fn name(&self) -> &'static str {
    "Insertion Sort"
  }
}

use algorithms::Algorithm;
use array::Array;

pub struct SelectionSort;

impl Algorithm for SelectionSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for i in 0..len - 1 {
      array.set_color(i, [0.0, 1.0, 0.0, 0.7]);

      let mut min_i = i;
      for j in i + 1..len {
        if array.get(j) < array.get(min_i) {
          if min_i != i {
            array.reset_color(min_i);
          }

          min_i = j;
          array.set_color(min_i, [0.0, 1.0, 0.0, 0.7]);
        }

        array.wait(2);
      }

      array.wait(20);

      array.swap(i, min_i);
      array.reset_color(i);
      array.reset_color(min_i);
    }
  }

  fn name(&self) -> &'static str {
    "Selection Sort"
  }
}

use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct GnomeSort;

impl Algorithm for GnomeSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    let mut i = 0;
    while i < len {
      array.set_color(i, [0.0, 1.0, 0.0, 0.8]);
      delay(5);
      array.reset_color(i);

      if i == 0 || array.get(i) >= array.get(i - 1) {
        i += 1;
      } else {
        array.swap(i, i - 1);
        i -= 1;
      }
    }
  }

  fn name(&self) -> &'static str {
    "Gnome Sort"
  }
}

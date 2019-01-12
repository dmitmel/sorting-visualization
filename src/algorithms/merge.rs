//! [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)

use super::{Algorithm, Array};

/// [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)
pub struct MergeSort;

impl Algorithm for MergeSort {
  fn sort(&self, array: Array) {
      let len = array.len();
      let mut tmp_array: Vec<u32> = (1..=len as u32).collect();
      self.split(&array, 0, len as isize - 1, &mut tmp_array);
  }

  fn name(&self) -> String {
    "Merge sort".to_string()
  }
}

impl MergeSort {
    fn split(&self, array: &Array, l_index: isize, r_index: isize, tmp_array: &mut Vec<u32>) {
        if  r_index - l_index > 1 {
            let mid = (l_index + r_index) / 2;
            self.split(array, l_index, mid, tmp_array);
            self.split(array, mid, r_index, tmp_array);
        }
    }

    fn merge() {

    }
}

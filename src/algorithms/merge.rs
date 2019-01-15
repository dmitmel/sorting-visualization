//! [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)

use super::{Algorithm, Array};

/// [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)
pub struct MergeSort;

impl Algorithm for MergeSort {
  fn sort(&self, array: Array) {
      let len = array.len();
      let last = len as usize - 1;
      self.split(&array, 0, last);
  }

  fn name(&self) -> String {
    "Merge sort".to_string()
  }
}

impl MergeSort {
    fn split(&self, array: &Array, l: usize, r: usize) {
        if  l < r {
            let m: usize = l + (r - l) / 2;
            self.split(array, l, m);
            self.split(array, m + 1, r);
            self.merge(array, l, m, r);
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn merge(&self, array: &Array, l: usize, m: usize, r: usize) {
      let mut i:usize = 0;
      let mut j:usize = 0;
      let mut k:usize = l;

      let n1:usize = m - l + 1;
      let n2:usize = r - m;

      let mut l_arr: Vec<u32> = (0..n1 as u32).collect();
      let mut r_arr: Vec<u32> = (0..n2 as u32).collect();

      for i in 0..n1 {
          l_arr[i] = array.get(l + i);
      }

      for j in 0..n2 {
          r_arr[j] = array.get(m + 1 + j);
      }

      while i < n1 && j < n2 {
          if l_arr[i] <= r_arr[j] {
              array.set_color(k, [0.0, 1.0, 0.0, 0.8]);
              array.set(k, l_arr[i]);
              array.wait(5);
              array.reset_color(k);
              i += 1;
          } else {
              array.set(k, r_arr[j]);
              array.wait(5);
              array.reset_color(k);
              j += 1;
          }
          k += 1;
      }

      while i < n1 {
          array.set_color(k, [0.0, 1.0, 0.0, 0.8]);
          array.set(k, l_arr[i]);
          array.wait(5);
          array.reset_color(k);
          i += 1;
          k += 1;
      }

      while j < n2 {
          array.set_color(k, [0.0, 1.0, 0.0, 0.8]);
          array.set(k, r_arr[j]);
          array.wait(5);
          array.reset_color(k);
          j += 1;
          k += 1;
      }
    }
}

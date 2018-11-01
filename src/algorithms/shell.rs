//! [Shell sort](https://en.wikipedia.org/wiki/Shellsort)

use super::{Algorithm, Array};

/// [Shell sort](https://en.wikipedia.org/wiki/Shellsort)
pub struct Shellsort;

impl Algorithm for Shellsort {
  fn sort(&self, array: Array) {
    let len = array.len() as i64;

    let mut gap = 1;

    while gap < (len >> 1) {
      gap = (gap << 1) + 1;
    }

    while gap >= 1 {
      let mut i = gap;
      while i < len {
        let mut k = i - gap;
        let mut j = i;
        while j >= gap && array.get(j as usize) < array.get(k as usize) {
          array.swap(j as usize, k as usize);
          array.wait(10);
          j = k;
          k -= gap;
        }
        array.wait(10);
        i += 1;
      }
      gap >>= 1;
    }
  }

  fn name(&self) -> String {
    "Shell sort".to_string()
  }
}

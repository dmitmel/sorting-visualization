//! [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)

use super::{Algorithm, Array};

/// [Merge sort](https://en.wikipedia.org/wiki/Merge_sort)
pub struct MergeSort;

impl Algorithm for MergeSort {
  fn sort(&self, array: Array) {
    merge_sort(&array, 0, array.len() - 1);
  }

  fn name(&self) -> String {
    "Merge sort".to_string()
  }
}

fn merge_sort(array: &Array, left: usize, right: usize) {
  if left < right {
    let middle = (left + right) / 2;
    array.set_color(middle, [1.0, 0.0, 0.0, 1.0]);

    for i in left..middle {
      array.set_color(i, [0.0, 1.0, 0.0, 0.3]);
    }
    merge_sort(array, left, middle);

    for i in middle + 1..=right {
      array.set_color(i, [0.0, 0.0, 1.0, 0.3]);
    }
    merge_sort(array, middle + 1, right);

    merge(array, left, middle, right);

    for i in left..=right {
      array.reset_color(i);
    }
  }
}

fn merge(array: &Array, left: usize, middle: usize, right: usize) {
  let left_size = middle - left + 1;
  let right_size = right - middle;

  let left_array = sub_array(array, left, left_size);
  let right_array = sub_array(array, middle + 1, right_size);

  let mut i = 0;
  let mut j = 0;
  let mut k = left;
  let mut prev_k = k;

  while i < left_size && j < right_size {
    array.reset_color(prev_k);
    array.set_color(k, [1.0, 0.0, 0.0, 1.0]);
    prev_k = k;

    if left_array[i] <= right_array[j] {
      array.set(k, left_array[i]);
      i += 1;
    } else {
      array.set(k, right_array[j]);
      j += 1;
    }
    k += 1;
    array.wait(20);
  }

  array.reset_color(prev_k);

  while i < left_size {
    array.set(k, left_array[i]);
    i += 1;
    k += 1;
    array.wait(20);
  }

  while j < right_size {
    array.set(k, right_array[j]);
    j += 1;
    k += 1;
    array.wait(20);
  }
}

fn sub_array(array: &Array, begin: usize, size: usize) -> Vec<u32> {
  let mut arr = vec![0; size];
  for i in 0..size {
    arr[i] = array.get(begin + i);
    array.wait(10);
  }
  arr
}

use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct Quicksort;

impl Algorithm for Quicksort {
  fn sort(&self, array: Array) {
    self.sort_slice(&array, 0, array.len() as isize - 1);
  }

  fn name(&self) -> &'static str {
    "Quicksort"
  }
}

impl Quicksort {
  fn sort_slice(&self, array: &Array, low: isize, high: isize) {
    if low < high {
      let pivot = self.partition(array, low, high);
      array.set_highlighted_index(pivot as usize);
      self.sort_slice(array, low, pivot - 1);
      self.sort_slice(array, pivot + 1, high);
    }
  }

  fn partition(&self, array: &Array, low: isize, high: isize) -> isize {
    let pivot = array.get(high as usize);
    let mut i = low;
    for j in low..high {
      if array.get(j as usize) <= pivot {
        array.swap(i as usize, j as usize);
        delay(30);
        i += 1;
      }
    }
    array.swap(i as usize, high as usize);
    i
  }
}

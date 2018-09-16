use algorithms::Algorithm;
use array::Array;
use utils::delay;

pub struct CycleSort;

impl Algorithm for CycleSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    for cycle_start in 0..len - 1 {
      array.set_highlighted_index(cycle_start);

      let mut item = array.get(cycle_start);
      let mut sorted_index = self.find_sorted_index(&array, cycle_start, item);

      if sorted_index == cycle_start {
        continue;
      }

      while item == array.get(sorted_index) {
        sorted_index += 1;
      }

      let tmp = item;
      item = array.get(sorted_index);
      array.set(sorted_index, tmp);

      while sorted_index != cycle_start {
        array.set_highlighted_index(sorted_index);
        sorted_index = cycle_start;

        for i in cycle_start + 1..len {
          if array.get(i) < item {
            sorted_index += 1;
            delay(5);
          }
        }

        while item == array.get(sorted_index) {
          sorted_index += 1;
          delay(5);
        }

        let tmp = item;
        item = array.get(sorted_index);
        array.set(sorted_index, tmp);
      }
    }
  }

  fn name(&self) -> &'static str {
    "Cycle Sort"
  }
}

impl CycleSort {
  fn find_sorted_index(
    &self,
    array: &Array,
    cycle_start: usize,
    item: u32,
  ) -> usize {
    let len = array.len();

    let mut sorted_index = cycle_start;
    for i in cycle_start + 1..len {
      if array.get(i) < item {
        sorted_index += 1;
        delay(3);
      }
    }

    sorted_index
  }
}

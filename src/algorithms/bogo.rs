

use super::{Algorithm, Array};

pub struct BogoSort;

impl Algorithm for BogoSort {
  fn sort(&self, array: Array) {
    let len = array.len();
    let mut done = false;
    while !done {
      let mut i = 0; 
      array.set_color(i, [0.0, 1.0, 0.0, 0.8]);
      array.wait(5);
      array.reset_color(i);
      array.shuffle();
      
      loop{
        if i == 0 || array.get(i) >= array.get(i - 1) {
          i += 1;
        } else {
          break;
        }
      }
    }
  }

  fn name(&self) -> String {
    "Bogo sort".to_string()
  }
}

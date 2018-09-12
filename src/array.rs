use std::sync::{Arc, Mutex, MutexGuard};

use app::State;

pub struct Array(Arc<Mutex<State>>);

impl Array {
  pub fn new(state: Arc<Mutex<State>>) -> Self {
    Array(state)
  }

  fn lock(&self) -> MutexGuard<'_, State> {
    self.0.lock().unwrap()
  }

  pub fn len(&self) -> usize {
    let state = self.lock();
    state.array.len()
  }

  pub fn get(&self, index: usize) -> u32 {
    let mut state = self.lock();
    let value = state.array[index];

    let time = state.time;
    state.array_accesses.push(ArrayAccess { time, index });

    value
  }

  pub fn set(&self, index: usize, value: u32) {
    let mut state = self.lock();
    state.array[index] = value;
  }

  pub fn swap(&self, a: usize, b: usize) {
    let mut state = self.lock();
    state.array.swap(a, b);
  }
}

pub struct ArrayAccess {
  pub time: f64,
  pub index: usize,
}

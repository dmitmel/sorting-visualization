use std::sync::{Arc, Mutex, MutexGuard};

pub type Value = u32;

#[derive(Debug)]
pub struct Array(Arc<Mutex<Vec<Value>>>);

impl Array {
  pub fn new(values: Vec<Value>) -> Self {
    Array(Arc::new(Mutex::new(values)))
  }

  pub fn lock(&self) -> MutexGuard<'_, Vec<Value>> {
    self.0.lock().unwrap()
  }

  pub fn len(&self) -> usize {
    let values = self.lock();
    values.len()
  }

  pub fn get(&self, index: usize) -> Value {
    let values = self.lock();
    values[index]
  }

  pub fn set(&self, index: usize, value: Value) {
    let mut values = self.lock();
    values[index] = value
  }

  pub fn swap(&self, a: usize, b: usize) {
    let mut values = self.lock();
    values.swap(a, b)
  }
}

impl Clone for Array {
  fn clone(&self) -> Array {
    Array(self.0.clone())
  }
}

unsafe impl Send for Array {}
unsafe impl Sync for Array {}

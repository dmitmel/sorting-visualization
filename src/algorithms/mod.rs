use array::Array;

pub trait Algorithm {
  fn sort(&self, array: Array);
}

pub mod bubble;
pub use self::bubble::BubbleSort;

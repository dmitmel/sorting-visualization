use array::Array;

pub trait Algorithm {
  fn sort(&self, array: Array);
}

macro_rules! reexport {
  ($($name:ident),*) => ($(
    pub mod $name;
    pub use self::$name::*;
  )*);
}

reexport![bubble, gnome, insertion, selection];

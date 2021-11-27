//! TODO
//!
//! # Lua API
//!
//! <big>_Note:_ **indexes start at zero!**</big>
//!
//! ## global `array`
//!
//! | Rust                                                | Lua equivalent                  |
//! | --------------------------------------------------- | ------------------------------- |
//! | [`array.wait(ms)`](Array::wait)                     | `array:wait(ms)`                |
//! | [`array.len()`](Array::len)                         | `#array`                        |
//! | [`array.get(index)`](Array::get)                    | `array[index]`                  |
//! | [`array.set(index, value)`](Array::set)             | `array[index] = value`          |
//! | [`array.swap(a, b)`](Array::swap)                   | `array:swap(a, b)`              |
//! | [`array.reset_color(index)`](Array::reset_color)    | `array:reset_color(index)`      |
//! | [`array.set_color(index, color)`](Array::set_color) | `array:set_color(index, color)` |
//!
//! ## utilities
//!
//! ### `range(from, to)`

use std::fs;
use std::io::Result as IoResult;
use std::path::PathBuf;

use rlua::prelude::*;

use super::{Algorithm, Array};

const UTILS_SOURCE: &[u8] = include_bytes!("../../assets/utils.lua");
const UTILS_PATH: &str = "<internal>/utils.lua";

pub struct LuaAlgorithm {
  pub path: PathBuf,
  source: Vec<u8>,
}

impl LuaAlgorithm {
  pub fn load(path: PathBuf) -> IoResult<Self> {
    let source = fs::read(&path)?;
    Ok(Self { path, source })
  }

  fn try_sort(&self, array: Array) -> LuaResult<()> {
    let lua = Lua::new();

    // load our built-in utilities
    lua.exec(UTILS_SOURCE, Some(UTILS_PATH))?;

    // load the algorithm code as a function
    let path = format!("{}", self.path.display());
    let main: LuaFunction = lua.load(&self.source, Some(&path))?;

    // setup array
    lua.globals().set("array", array)?;

    // run the algorithm
    main.call(())
  }
}

impl Algorithm for LuaAlgorithm {
  fn sort(&self, array: Array) {
    if let Err(error) = self.try_sort(array) {
      crate::handle_error(error.into());
    }
  }

  fn name(&self) -> String {
    format!("{}", self.path.display())
  }
}

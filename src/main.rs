#![cfg_attr(feature = "doc", feature(external_doc))]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use failure::{Error, ResultExt};

use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;

mod algorithms;
mod app;
mod array;
mod cli;
mod state;

use crate::app::App;
use crate::cli::{Options, Order};

/// Required version of OpenGL.
///
/// _Note:_ change this to [`OpenGL::V2_1`] if it doesn't work.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Title of the main window.
const WINDOW_TITLE: &str = clap::crate_name!();
/// Initial size of the main window.
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
  if let Err(error) = run() {
    eprintln!("error: {}", error);

    for cause in error.iter_causes() {
      eprintln!("caused by: {}", cause);
    }

    eprintln!("{}", error.backtrace());
    eprintln!(
      "note: Run with `RUST_BACKTRACE=1` if you don't see a backtrace."
    );

    use std::process;
    process::exit(1);
  }
}

fn run() -> Result<(), Error> {
  let Options {
    algorithm,
    length,
    order,
    speed,
  } = cli::parse_options();

  let window_title = format!("{} - {}", WINDOW_TITLE, algorithm.name());
  let mut window: Window = WindowSettings::new(window_title, WINDOW_SIZE)
    .opengl(OPENGL_VERSION)
    .exit_on_esc(true)
    .vsync(true)
    .build()
    // convert `Result<_, String>` to `Result<_, Error>`
    .map_err(|e| failure::format_err!("{}", e))
    .context("couldn't create window")?;
  let mut gl = GlGraphics::new(OPENGL_VERSION);

  let mut array: Vec<u32> = (1..=length).collect();

  match order {
    Order::Sorted => {}
    Order::Reversed => array.reverse(),
    Order::Shuffled => {
      use rand::{thread_rng, Rng};
      thread_rng().shuffle(&mut array);
    }
  }

  // load font for the status text
  let font = include_bytes!("../assets/Menlo-Regular.ttf");
  let mut glyphs = GlyphCache::from_bytes(font, (), TextureSettings::new())
    // `GlyphCache::from_bytes` returns `Err(())` when an error occurs, so it's
    // replaced with an error with a meaningful message here
    .map_err(|_| failure::format_err!("couldn't load font"))?;

  // preload printable ASCII chars for faster rendering
  glyphs
    .preload_printable_ascii(app::STATUS_TEXT_FONT_SIZE)
    // convert `Result<_, String>` to `Result<_, Error>`
    .map_err(|e| failure::format_err!("{}", e))
    .context("couldn't preload printable ASCII chars")?;

  let mut app = App::init(algorithm, array, speed);

  println!("Press [Space] to pause/resume");
  println!("Press [Up]    to speed up");
  println!("Press [Down]  to slow down");
  println!();

  let mut events = Events::new(EventSettings::new());
  while let Some(event) = events.next(&mut window) {
    match event {
      Event::Loop(Loop::Render(args)) => app.render(args, &mut gl, &mut glyphs),
      Event::Loop(Loop::Update(args)) => app.update(args),
      Event::Input(Input::Button(args)) => app.button(args),
      _ => {}
    }
  }

  Ok(())
}

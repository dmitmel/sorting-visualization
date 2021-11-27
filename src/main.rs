#[macro_use]
extern crate clap;

extern crate rand;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2;
extern crate sdl2_window;

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

use app::App;
use cli::{Options, Order};

/// Required version of OpenGL.
///
/// _Note:_ change this to [`OpenGL::V2_1`] if it doesn't work.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Title of the main window.
const WINDOW_TITLE: &str = "sorting-visualization";
/// Initial size of the main window.
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
  let Options {
    algorithm,
    length,
    order,
    speed,
  } = cli::parse_options();

  let title = format!("{} - {}", WINDOW_TITLE, algorithm.name());
  let mut window: Window = WindowSettings::new(title, WINDOW_SIZE)
    .opengl(OPENGL_VERSION)
    .exit_on_esc(true)
    .vsync(true)
    .build()
    .expect("couldn't create window");
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

  use sdl2::audio::{AudioCallback, AudioSpecDesired};
  let desired_spec = AudioSpecDesired {
    freq: Some(44_100),
    channels: Some(1), // mono
    samples: None,     // default sample size
  };

  struct Wave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
  }

  impl AudioCallback for Wave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
      // for x in out.iter_mut() {
      for (i, x) in out.iter_mut().rev().enumerate() {
        // *x = self.phase.sin() * self.volume;
        // self.phase = (self.phase + self.phase_inc) % 1.0;
        *x = i as f32;
      }
    }
  }

  let audio_subsystem = window.sdl_context.audio().unwrap();
  let device = audio_subsystem
    .open_playback(None, &desired_spec, |spec| {
      println!("{:?}", spec);
      Wave {
        phase_inc: 60000.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.25,
      }
    })
    .unwrap();

  device.resume();

  let mut app = App::init(algorithm, array, speed);

  println!("Press [Space] to pause/resume the animation");
  println!("Press [Up]    to speed up the animation");
  println!("Press [Down]  to slow down the animation");
  println!();

  let mut events = Events::new(EventSettings::new());
  while let Some(event) = events.next(&mut window) {
    match event {
      Event::Loop(Loop::Render(args)) => app.render(&mut gl, args),
      Event::Loop(Loop::Update(args)) => app.update(args),
      Event::Input(Input::Button(args)) => app.button(args),
      _ => {}
    }
  }
}

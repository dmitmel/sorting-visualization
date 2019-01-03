//! Command-line interface and command-line argument parsing. Uses [clap] under
//! the hood.

use crate::algorithms;
use crate::algorithms::Algorithm;

/// [Internal name](clap::Arg::with_name) of the
/// [algorithm](Options::algorithm) argument which is used to
/// [get its value](clap::ArgMatches::value_of).
const ALGORITHM_ARG: &str = "ALGORITHM";
/// [Internal name](clap::Arg::with_name) of the
/// [`--length`/`-l`](Options::length) option which is used to
/// [get its value](clap::ArgMatches::value_of).
const LENGTH_OPT: &str = "LENGTH";
/// [Internal name](clap::Arg::with_name) of the
/// [`--order`/`-o`](Options::order) option which is used to
/// [get its value](clap::ArgMatches::value_of).
const ORDER_OPT: &str = "ORDER";
/// [Internal name](clap::Arg::with_name) of the
/// [`--speed`/`-s`](Options::speed) option which is used to
/// [get its value](clap::ArgMatches::value_of).
const SPEED_OPT: &str = "SPEED";
/// [Internal name](clap::SubCommand::with_name) of the
/// [list] subcommand which is used to
/// [return list of available algorithms](clap::ArgMatches::is_present).
const LIST_SUB: &str = "list";

/// Contains all options that can be provided by a user using the CLI.
pub struct Options {
  /// Instance of a sorting [algorithm](Algorithm) struct.
  pub algorithm: Box<dyn Algorithm + Send>,
  /// Number of elements in the [array](crate::array::Array).
  pub length: u32,
  /// Order of elements in the [array](crate::array::Array).
  pub order: Order,
  /// [Speed](crate::state::State::speed) factor.
  pub speed: f64,
}

/// Order of elements in the [array](crate::array::Array).
pub enum Order {
  /// Sorted in the ascending order.
  Sorted,
  /// Sorted in the descending order.
  Reversed,
  /// [Shuffled](rand::Rng::shuffle).
  Shuffled,
}

/// Parses command-line arguments into [`Options`]. **If the help/version is
/// printed or an error occurs, it will be displayed to the user and the process
/// will exit.**
///
/// _See_ [`clap::App.get_matches`](clap::App::get_matches)
pub fn parse_options() -> Options {
  use clap::*;

  let parser = app_from_crate!()
    .setting(AppSettings::NextLineHelp)
    .setting(AppSettings::ColoredHelp)
    .arg(
      Arg::with_name(ALGORITHM_ARG)
        .help("Sets sorting algorithm")
        .possible_values(&[
          "bubble",
          "cycle",
          "gnome",
          "insertion",
          "quicksort",
          "selection",
          "shell",
        ])
        .case_insensitive(true),
    )
    .arg(
      Arg::with_name(LENGTH_OPT)
        .short("l")
        .long("length")
        .help("Sets number of elements in the array")
        .default_value("100"),
    )
    .arg(
      Arg::with_name(ORDER_OPT)
        .short("o")
        .long("order")
        .help("Sets order of elements in the array")
        .possible_values(&["sorted", "reversed", "shuffled"])
        .case_insensitive(true)
        .default_value("shuffled"),
    )
    .arg(
      Arg::with_name(SPEED_OPT)
        .short("s")
        .long("speed")
        .help("Sets animation speed")
        .default_value("1.0"),
    )
    .subcommand(
      SubCommand::with_name(LIST_SUB)
        .about("List all available algorithms")
        .help("List of all available algotirhms"),
    );

  let matches = parser.get_matches();

  match matches.subcommand() {
    (LIST_SUB, Some(_m)) => display_algorithm_list(),
    _ => (),
  }

  fn display_algorithm_list() {
    extern crate glob;
    use glob::glob;
    use std::process::exit;

    println!("List of available algorithms:");

    let absolute_path = "src/algorithms/";

    for e in glob(&format!("./{}*", absolute_path))
      .expect("Failed to read glob pattern")
    {
      println!(
        "{}",
        e.unwrap()
          .display()
          .to_string()
          .replace(absolute_path, "")
          .replace(".rs", "")
      );
    }

    exit(1);
  }

  // all option values can be safely unwrapped here because their corresponding
  // options are either required or have a default value
  Options {
    algorithm: match matches.value_of(ALGORITHM_ARG).unwrap() {
      "bubble" => Box::new(algorithms::BubbleSort),
      "cycle" => Box::new(algorithms::CycleSort),
      "gnome" => Box::new(algorithms::GnomeSort),
      "insertion" => Box::new(algorithms::InsertionSort),
      "quicksort" => Box::new(algorithms::Quicksort),
      "selection" => Box::new(algorithms::SelectionSort),
      "shell" => Box::new(algorithms::Shellsort),
      _ => unreachable!(),
    },

    length: value_t_or_exit!(matches, LENGTH_OPT, u32),

    order: match matches.value_of(ORDER_OPT).unwrap() {
      "sorted" => Order::Sorted,
      "reversed" => Order::Reversed,
      "shuffled" => Order::Shuffled,
      _ => unreachable!(),
    },

    speed: value_t_or_exit!(matches, SPEED_OPT, f64),
  }
}

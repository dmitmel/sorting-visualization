# sorting-visualization

[![Travis (.org)](https://img.shields.io/travis/dmitmel/sorting-visualization.svg?style=flat-square)](https://travis-ci.org/dmitmel/sorting-visualization)
[![contributors welcome](https://img.shields.io/badge/contributors-welcome-brightgreen.svg?style=flat-square)](https://github.com/dmitmel/sorting-visualization/pulls)

A [Rust](https://www.rust-lang.org/) program for visualizing sorting algorithms which uses [Piston](http://www.piston.rs/) for graphics. Inspired by [**Hopson97/Sort-Algorithm-Visualiser**](https://github.com/Hopson97/Sort-Algorithm-Visualiser).

[![Demo](https://i.imgur.com/jyPDiWX.gif)](https://gist.github.com/dmitmel/f8664421b547577065912c3246f4c1e9)

## Installation

```bash
git clone https://github.com/dmitmel/sorting-visualization
cd sorting-visualization
cargo install
```

You can use `cargo run` if you don't want to install the binary system-wide.

**Note:** compilation takes a lot of time

## Usage

```bash
# see 'Features' for the list of supported algorithms and their IDs
sorting-visualization <algorithm>
# set length of the array
sorting-visualization <algorithm> --length <number>
# set order of elements in the array
sorting-visualization <algorithm> --order <sorted|reversed|shuffled>
```

## Features

- CLI
- Supports different algorithms (IDs are in brackets):
  - [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort) \[`bubble`\]
  - [Cycle sort](https://en.wikipedia.org/wiki/Cycle_sort) \[`cycle`\]
  - [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort) \[`gnome`\]
  - [Insertion sort](https://en.wikipedia.org/wiki/Insertion_sort) \[`insertion`\]
  - [Quicksort](https://en.wikipedia.org/wiki/Quicksort) \[`quicksort`\]
  - [Selection sort](https://en.wikipedia.org/wiki/Selection_sort) \[`selection`\]
- Animation controls:
  - <kbd>Space</kbd> - pause/resume
  - <kbd>&uparrow;</kbd> - 2x faster
  - <kbd>&downarrow;</kbd> - 2x slower
- Easy-to-use algorithm API
- Algorithms can highlight important array elements
- Code is well-documented

## Building docs

```bash
cargo +nightly doc --document-private-items --open
```

Nightly Rust is required for building docs because the `--document-private-items` is currently unstable. This option is very useful when developing an application (not library) because you would probably like to see the documentation of the whole codebase.

## TODO

1. Ask someone to proof-read the code
2. Draw the animation state as text in the window instead of printing it to the console
3. Add a CLI option to list available algorithms
4. User-friendly GUI
5. More algorithms
6. Sound?

## Contribute

PRs are appreciated!

## License

[MIT](https://github.com/dmitmel/sorting-visualization/blob/master/LICENSE) © [Dmytro Meleshko](https://github.com/dmitmel)

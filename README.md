# Conway's Game of Life

_In the 1970s, John Horton Conway came up with a cellular automaton that simulates life within a simple 2-dimensional universe and is dubbed [Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life). It is effectively a zero-player game, as the universe's evolution is solely determined by its current state rather than any user input._

This repository contains a simple implementation of this game, written in Rust using [`ggez`](https://crates.io/crates/ggez).
I mainly wanted to make this to become a little more familiar with the ggez engine and [cellular automata](https://en.wikipedia.org/wiki/Cellular_automaton) in general.

## Usage

To get this bad boy up and running snag the code via `git clone` and `cargo run` it from your terminal.
If no arguments are supplied, the program will generate a random universe to simulate.
You can alternatively supply a `u64` string as seed to the universe generator.

## License

This work is licensed under the **MIT** license. For more information have a look at the license file in this repository.

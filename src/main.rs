use ggez::event;
use ggez::{conf, ContextBuilder};
use std::env;

mod universe;

use universe::Universe;

static DIM_X: f32 = 800.0;
static DIM_Y: f32 = 600.0;
static SIZE_X: usize = 80;
static SIZE_Y: usize = 60;

fn main() {
    let mut seed = None;
    let mut args = env::args();
    args.next();

    for arg in args {
        if arg == "--help" || arg == "-h" {
            println!("Conways Game of Life\nUsage: ./game-of-life [seed]\n\n  seed          64-Bit unsigned integer to seed the random number generator");
            return;
        } else {
            let s = arg.parse().unwrap();
            seed = Some(s);
        }
    }

    let (mut context, mut event_loop) = ContextBuilder::new("Game of Life", "Felix Wittwer")
        .window_mode(conf::WindowMode::default().dimensions(DIM_X, DIM_Y))
        .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
        .build()
        .unwrap();

    let mut universe = Universe::initialize((SIZE_X, SIZE_Y), seed);

    event::run(&mut context, &mut event_loop, &mut universe).unwrap();
}

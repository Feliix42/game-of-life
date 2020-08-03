use ggez::event;
use ggez::{conf, ContextBuilder};

mod universe;

use universe::Universe;

static DIM_X: f32 = 800.0;
static DIM_Y: f32 = 600.0;
static SIZE_X: usize = 80;
static SIZE_Y: usize = 60;

fn main() {
    let (mut context, mut event_loop) = ContextBuilder::new("Game of Life", "Felix Wittwer")
        .window_mode(conf::WindowMode::default().dimensions(DIM_X, DIM_Y))
        .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
        .build()
        .unwrap();

    let mut universe = Universe::initialize((SIZE_X, SIZE_Y), None);

    event::run(&mut context, &mut event_loop, &mut universe).unwrap();
}

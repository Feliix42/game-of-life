use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{DrawParam, Image};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

/// State of an individual cell
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Live,
    Dead,
}

/// The universe the game is set in
pub struct Universe {
    /// dimensions of the universe
    dimensions: (usize, usize),
    /// the neighborhood itself: x indexes the first vector, y the second.
    neighborhood: Vec<Vec<CellState>>,
}

impl Universe {
    /// Initialize a universe with the given dimensions either using a provided seed or using a randomly generated seed.
    pub fn initialize(dimensions: (usize, usize), seed: Option<u64>) -> Self {
        let (x, y) = dimensions;
        let seed = seed.unwrap_or(rand::random());
        let mut rng = SmallRng::seed_from_u64(seed);

        let mut neighborhood = vec![Vec::with_capacity(y); x];

        for idx in 0..x {
            for _ in 0..y {
                let state = match rng.next_u32() % 2 {
                    0 => CellState::Dead,
                    1 => CellState::Live,
                    _ => panic!("Invalid RNG result for generation"),
                };
                neighborhood[idx].push(state);
            }
        }

        Self {
            dimensions,
            neighborhood,
        }
    }

    /// Compute the next generation of the universe.
    pub fn advance_generation(&mut self) {
        // clone the neighborhood to be able to just keep the non-changed data
        let mut new_neighborhood = self.neighborhood.clone();

        let (x_max, y_max) = self.dimensions;
        for x in 0..x_max {
            for y in 0..y_max {
                // check all directions, count # of neighbors
                let mut life_neighbors: u8 = 0;
                if x > 0 {
                    if y > 0 && self.neighborhood[x - 1][y - 1] == CellState::Live {
                        life_neighbors += 1;
                    }
                    if self.neighborhood[x - 1][y] == CellState::Live {
                        life_neighbors += 1;
                    }
                    if y < (y_max - 1) && self.neighborhood[x - 1][y + 1] == CellState::Live {
                        life_neighbors += 1;
                    }
                }

                if y > 0 && self.neighborhood[x][y - 1] == CellState::Live {
                    life_neighbors += 1;
                }
                if y < (y_max - 1) && self.neighborhood[x][y + 1] == CellState::Live {
                    life_neighbors += 1;
                }

                if x < (x_max - 1) {
                    if y > 0 && self.neighborhood[x + 1][y - 1] == CellState::Live {
                        life_neighbors += 1;
                    }
                    if self.neighborhood[x + 1][y] == CellState::Live {
                        life_neighbors += 1;
                    }
                    if y < (y_max - 1) && self.neighborhood[x + 1][y + 1] == CellState::Live {
                        life_neighbors += 1;
                    }
                }

                if self.neighborhood[x][y] == CellState::Live {
                    if life_neighbors != 2 && life_neighbors != 3 {
                        new_neighborhood[x][y] = CellState::Dead;
                    }
                } else {
                    if life_neighbors == 3 {
                        new_neighborhood[x][y] = CellState::Live;
                    }
                }
            }
        }

        self.neighborhood = new_neighborhood;
    }
}

impl EventHandler for Universe {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.advance_generation();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let box_shape_thingy = Image::solid(ctx, 10, graphics::BLACK).unwrap();
        let mut batch = SpriteBatch::new(box_shape_thingy);

        let (x_max, y_max) = self.dimensions;
        for x in 0..x_max {
            let x_coord = x as f32 * 10.0;
            for y in 0..y_max {
                if self.neighborhood[x][y] == CellState::Live {
                    batch.add(DrawParam::new().dest(Point2::new(x_coord, y as f32 * 10.0)));
                }
            }
        }

        graphics::draw(ctx, &batch, DrawParam::default()).unwrap();

        graphics::present(ctx)
    }
}

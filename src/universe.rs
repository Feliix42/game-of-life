// use rand;
use rand::{RngCore, SeedableRng};
use rand::rngs::SmallRng;

/// State of an individual cell
#[derive(Clone, Copy)]
pub enum CellState {
    Live,
    Dead
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
            neighborhood
        }
    }

    /// Compute the next generation of the universe.
    pub fn advance_generation(&mut self) {
        unimplemented!()
    }
}

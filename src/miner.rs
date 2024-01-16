use crate::block::Block;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct MineResult {
    pub block: Block,
    pub delay: f64,
}

pub struct Uniform {
    rng: ChaCha8Rng,
    delay_from: f64,
    delay_to: f64,
}

impl Uniform {
    pub fn new(seed: u64, delay_from: f64, delay_to: f64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            delay_from,
            delay_to,
        }
    }

    pub fn mine(&mut self) -> MineResult {
        MineResult {
            block: Block::default(),
            delay: self.rng.gen_range(self.delay_from..=self.delay_to),
        }
    }
}

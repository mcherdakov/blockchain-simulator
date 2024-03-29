use crate::block::Block;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct MineResult {
    pub block: Block,
    pub delay: f64,
}

pub struct UniformMiner {
    rng: ChaCha8Rng,
    delay_from: f64,
    delay_to: f64,
}

impl UniformMiner {
    pub fn new(seed: u64, delay_from: f64, delay_to: f64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            delay_from,
            delay_to,
        }
    }

    pub fn mine(&mut self, prev_block: &Block) -> MineResult {
        MineResult {
            block: Block::new(&mut self.rng, prev_block),
            delay: self.rng.gen_range(self.delay_from..=self.delay_to),
        }
    }
}

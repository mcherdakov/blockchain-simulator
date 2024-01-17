use rand::distributions::{Alphanumeric, DistString};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct BlockID {
    hash: String,
}

impl BlockID {
    fn random(rng: &mut ChaCha8Rng) -> Self {
        Self {
            hash: Alphanumeric.sample_string(rng, 16),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Block {
    pub id: BlockID,
}

impl Block {
    pub fn random(rng: &mut ChaCha8Rng) -> Self {
        Self {
            id: BlockID::random(rng),
        }
    }
}
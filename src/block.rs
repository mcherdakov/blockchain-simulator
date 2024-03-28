use std::fmt::Debug;

use rand::distributions::{Alphanumeric, DistString};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;

pub type BlockHash = String;

#[derive(Clone, Serialize)]
pub struct Block {
    pub hash: BlockHash,
    pub prev_hash: BlockHash,
    pub height: u64,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.hash, self.prev_hash)
    }
}

impl Block {
    pub fn new(rng: &mut ChaCha8Rng, parent: &Block) -> Self {
        Self {
            hash: Self::random_hash(rng),
            prev_hash: parent.hash.clone(),
            height: parent.height + 1,
        }
    }

    pub fn genesis(rng: &mut ChaCha8Rng) -> Self {
        Self {
            hash: Self::random_hash(rng),
            prev_hash: String::from(""),
            height: 0,
        }
    }

    fn random_hash(rng: &mut ChaCha8Rng) -> BlockHash {
        BlockHash::from(&Alphanumeric.sample_string(rng, 4))
    }
}

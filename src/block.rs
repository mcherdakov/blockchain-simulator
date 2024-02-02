use rand::distributions::{Alphanumeric, DistString};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;

pub type BlockID = String;

#[derive(Clone, Debug, Serialize)]
pub struct Block {
    pub id: BlockID,
}

impl Block {
    pub fn random(rng: &mut ChaCha8Rng) -> Self {
        Self {
            id: BlockID::from(Alphanumeric.sample_string(rng, 16)),
        }
    }
}

use std::collections::HashMap;

use crate::block::{Block, BlockID};

#[derive(Debug)]
pub struct BlockStorage {
    blocks: HashMap<BlockID, Block>,
}

impl BlockStorage {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn add(&mut self, block: Block) {
        self.blocks.insert(block.id.clone(), block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn add() {
        let mut block_storage = BlockStorage::new();

        let mut rng = ChaCha8Rng::seed_from_u64(64);
        let block = crate::block::Block::random(&mut rng);
        let block_id = block.id.clone();

        block_storage.add(block);
        assert_eq!(1, block_storage.blocks.len());

        let storage_block = block_storage
            .blocks
            .get(&block_id)
            .expect("expected block to be inserted");
        assert_eq!(storage_block.id, block_id);
    }
}

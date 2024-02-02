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

    pub fn contains(&self, block_id: &BlockID) -> bool {
        self.blocks.contains_key(block_id)
    }

    pub fn block(&self, block_id: &BlockID) -> Option<Block> {
        self.blocks.get(block_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn create_block() -> Block {
        let mut rng = ChaCha8Rng::seed_from_u64(64);
        crate::block::Block::random(&mut rng)
    }

    #[test]
    fn add_inserts_value() {
        let mut block_storage = BlockStorage::new();

        let block = create_block();
        let block_id = block.id.clone();

        block_storage.add(block);
        assert_eq!(1, block_storage.blocks.len());

        let storage_block = block_storage
            .blocks
            .get(&block_id)
            .expect("expected block to be inserted");
        assert_eq!(storage_block.id, block_id);
    }

    #[test]
    fn contains_exists() {
        let mut block_storage = BlockStorage::new();

        let block = create_block();
        let block_id = block.id.clone();

        block_storage.add(block);
        assert!(block_storage.contains(&block_id));
    }

    #[test]
    fn contains_not_exists() {
        let block_storage = BlockStorage::new();
        let block_id = BlockID::from("test");

        assert!(!block_storage.contains(&block_id));
    }

    #[test]
    fn block_exists() {
        let mut block_storage = BlockStorage::new();

        let block = create_block();
        let block_id = block.id.clone();

        block_storage.add(block);
        assert_eq!(block_storage.block(&block_id).unwrap().id, block_id);
    }

    #[test]
    fn block_not_exists() {
        let block_storage = BlockStorage::new();
        let block_id = BlockID::from("test");

        assert!(block_storage.block(&block_id).is_none());
    }
}

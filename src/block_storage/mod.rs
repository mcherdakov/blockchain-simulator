use std::{cmp::max, collections::HashMap, fmt::Debug};

use crate::block::{Block, BlockHash};

pub struct BlockStorage {
    blocks: HashMap<BlockHash, Block>,
    tip_hash: BlockHash,
}

impl Debug for BlockStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut blocks_by_height: HashMap<u64, Vec<&Block>> = HashMap::new();

        let mut max_height = 0;
        for (_, v) in self.blocks.iter() {
            match blocks_by_height.get_mut(&v.height) {
                Some(blocks) => {
                    blocks.push(v);
                }
                None => {
                    blocks_by_height.insert(v.height, vec![v]);
                }
            };

            max_height = max(max_height, v.height);
        }

        writeln!(f)?;

        for i in 0..max_height {
            match blocks_by_height.get(&i) {
                Some(blocks) => {
                    writeln!(f, "{i}: {:?}", blocks)?;
                }
                None => {
                    writeln!(f, "{i}: empty")?;
                }
            };
        }

        Ok(())
    }
}

impl BlockStorage {
    pub fn new(genesis: Block) -> Self {
        let mut blocks = HashMap::new();
        let hash = genesis.hash.clone();
        blocks.insert(hash.clone(), genesis);

        Self {
            blocks,
            tip_hash: hash,
        }
    }

    pub fn add(&mut self, block: Block) {
        if self.tip_hash.eq(&block.prev_hash) {
            self.tip_hash = block.hash.clone();
        }

        self.blocks.insert(block.hash.clone(), block);
    }

    pub fn contains(&self, block_hash: &BlockHash) -> bool {
        self.blocks.contains_key(block_hash)
    }

    pub fn block(&self, block_hash: &BlockHash) -> Option<&Block> {
        self.blocks.get(block_hash)
    }

    pub fn tip(&self) -> &Block {
        self.blocks.get(&self.tip_hash).unwrap()
    }

    pub fn size(&self) -> usize {
        self.blocks.len()
    }
}

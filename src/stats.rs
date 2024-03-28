use crate::block::BlockHash;

#[derive(Default, Debug)]
pub struct NodeStats {
    pub mined_blocks: Vec<BlockHash>,
}

impl NodeStats {
    pub fn block_mined(&mut self, hash: &BlockHash) {
        self.mined_blocks.push(hash.clone())
    }
}


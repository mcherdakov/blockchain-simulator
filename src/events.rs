use crate::block::{Block, BlockHash};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct BlockMined {
    pub block: Block,
}

#[derive(Clone, Serialize)]
pub struct Inv {
    pub block_id: BlockHash,
}

#[derive(Clone, Serialize)]
pub struct GetData {
    pub block_id: BlockHash,
}

#[derive(Clone, Serialize)]
pub struct BlockData {
    pub block: Block,
}

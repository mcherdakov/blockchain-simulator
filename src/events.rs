use crate::block::{Block, BlockID};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct BlockMined {
    pub block: Block,
}

#[derive(Clone, Serialize)]
pub struct Inv {
    pub block_id: BlockID,
}

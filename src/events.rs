use crate::block::Block;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Ping {}

#[derive(Clone, Serialize)]
pub struct Pong {}

#[derive(Clone, Serialize)]
pub struct BlockMined {
    pub block: Block,
}

use std::collections::HashSet;

use crate::block::{Block, BlockID};
use crate::events::{BlockData, GetData, Inv};
use crate::miner::Uniform;
use crate::{block_storage::BlockStorage, events::BlockMined};
use dslab_core::{cast, Event, EventHandler, Id, SimulationContext};

const MINE_DELAY_FROM: f64 = 2.0;
const MINE_DELAY_TO: f64 = 5.0;

#[derive(Default, Debug)]
pub struct Stats {
    pub blocks_mined: u32,
}

pub struct Node {
    ctx: SimulationContext,
    stats: Stats,
    block_storage: BlockStorage,
    miner: Uniform,
    peers: HashSet<Id>,
}

impl Node {
    pub fn new(ctx: SimulationContext, seed: u64) -> Self {
        let seed = seed.wrapping_add(ctx.id() as u64);

        Self {
            ctx,
            stats: Stats::default(),
            block_storage: BlockStorage::new(),
            miner: Uniform::new(seed, MINE_DELAY_FROM, MINE_DELAY_TO),
            peers: HashSet::new(),
        }
    }

    pub fn add_peers(&mut self, peers: &[Id]) {
        self.peers.extend(peers.iter());
    }

    pub fn start(&mut self) {
        self.mine_block();
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn storage(&self) -> &BlockStorage {
        &self.block_storage
    }

    fn mine_block(&mut self) {
        let mine_result = self.miner.mine();
        self.ctx.emit(
            BlockMined {
                block: mine_result.block,
            },
            self.ctx.id(),
            mine_result.delay,
        );
    }

    fn handle_block_mined(&mut self, block: Block) {
        let block_id = block.id.clone();

        self.stats.blocks_mined += 1;
        self.block_storage.add(block);

        if self.ctx.time() < 10.0 {
            self.mine_block();
        }

        for &peer in self.peers.iter() {
            self.ctx.emit(
                Inv {
                    block_id: block_id.clone(),
                },
                peer,
                0.0,
            );
        }
    }

    fn handle_inv(&mut self, block_id: BlockID, src: u32) {
        if self.block_storage.contains(&block_id) {
            return;
        }

        self.ctx.emit(GetData { block_id }, src, 0.0);
    }

    fn handle_get_data(&self, block_id: BlockID, src: u32) {
        if let Some(block) = self.block_storage.block(&block_id) {
            self.ctx.emit(BlockData { block }, src, 0.0);
        }
    }

    fn handle_block_data(&mut self, block: Block, src: u32) {
        let block_id = block.id.clone();
        self.block_storage.add(block);

        for &peer in self.peers.iter().filter(|&&peer| peer != src) {
            self.ctx.emit(
                Inv {
                    block_id: block_id.clone(),
                },
                peer,
                0.0,
            );
        }
    }
}

impl EventHandler for Node {
    fn on(&mut self, event: Event) {
        cast!(match event.data {
            BlockMined { block } => {
                self.handle_block_mined(block);
            }
            Inv { block_id } => {
                self.handle_inv(block_id, event.src);
            }
            GetData { block_id } => {
                self.handle_get_data(block_id, event.src);
            }
            BlockData { block } => {
                self.handle_block_data(block, event.src);
            }
        })
    }
}

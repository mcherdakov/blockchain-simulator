use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::block::{Block, BlockHash};
use crate::events::{BlockData, GetData, Inv};
use crate::miner::UniformMiner;
use crate::network::UniformNetwork;
use crate::stats::NodeStats;
use crate::{block_storage::BlockStorage, events::BlockMined};
use dslab_core::{cast, Event, EventHandler, Id, SimulationContext};

const MINE_DELAY_FROM: f64 = 1.0;
const MINE_DELAY_TO: f64 = 10.0;

pub struct Node {
    ctx: SimulationContext,
    stats: NodeStats,
    peers: HashSet<Id>,
    block_storage: BlockStorage,
    miner: UniformMiner,
    network: Rc<RefCell<UniformNetwork>>,
}

impl Node {
    pub fn new(
        ctx: SimulationContext,
        seed: u64,
        genesis_block: Block,
        network: Rc<RefCell<UniformNetwork>>,
    ) -> Self {
        let seed = seed.wrapping_add((ctx.id() + 1) as u64);

        Self {
            ctx,
            stats: NodeStats::default(),
            peers: HashSet::new(),
            block_storage: BlockStorage::new(genesis_block),
            miner: UniformMiner::new(seed, MINE_DELAY_FROM, MINE_DELAY_TO),
            network,
        }
    }

    pub fn add_peers(&mut self, peers: &[Id]) {
        self.peers.extend(peers.iter());
    }

    pub fn start(&mut self) {
        self.mine_block();
    }

    pub fn stats(&self) -> &NodeStats {
        &self.stats
    }

    pub fn storage(&self) -> &BlockStorage {
        &self.block_storage
    }

    pub fn id(&self) -> Id {
        self.ctx.id()
    }

    fn mine_block(&mut self) {
        let mine_result = self.miner.mine(self.block_storage.tip());
        self.ctx.emit(
            BlockMined {
                block: mine_result.block,
            },
            self.ctx.id(),
            mine_result.delay,
        );
    }

    fn handle_block_mined(&mut self, block: Block) {
        if self.block_storage.tip().hash != block.prev_hash {
            return;
        }

        let block_hash = block.hash.clone();

        self.stats.block_mined(&block_hash);
        self.block_storage.add(block);

        for &peer in self.peers.iter() {
            self.network.borrow_mut().send(
                &self.ctx,
                Inv {
                    block_id: block_hash.clone(),
                },
                peer,
            );
        }

        self.mine_block();
    }

    fn handle_inv(&mut self, block_id: BlockHash, src: u32) {
        self.add_peers(&[src]);

        if self.block_storage.contains(&block_id) {
            return;
        }

        self.network
            .borrow_mut()
            .send(&self.ctx, GetData { block_id }, src);
    }

    fn handle_get_data(&self, block_id: BlockHash, src: u32) {
        if let Some(block) = self.block_storage.block(&block_id) {
            self.network.borrow_mut().send(
                &self.ctx,
                BlockData {
                    block: block.to_owned(),
                },
                src,
            );
        }
    }

    fn handle_block_data(&mut self, block: Block, src: u32) {
        let block_id = block.hash.clone();
        self.block_storage.add(block);

        for &peer in self.peers.iter().filter(|&&peer| peer != src) {
            self.network.borrow_mut().send(
                &self.ctx,
                Inv {
                    block_id: block_id.clone(),
                },
                peer,
            );
        }

        self.mine_block();
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

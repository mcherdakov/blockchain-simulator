use crate::events::BlockMined;
use crate::miner::Uniform;
use dslab_core::{cast, Event, EventHandler, SimulationContext};

const MINE_DELAY_FROM: f64 = 2.0;
const MINE_DELAY_TO: f64 = 5.0;

#[derive(Default, Debug)]
pub struct Data {
    pub blocks_mined: u32,
}

pub struct Node {
    ctx: SimulationContext,
    data: Data,
    miner: Uniform,
}

impl Node {
    pub fn new(ctx: SimulationContext, seed: u64) -> Self {
        let seed = seed.wrapping_add(ctx.id() as u64);

        Self {
            ctx,
            data: Data::default(),
            miner: Uniform::new(seed, MINE_DELAY_FROM, MINE_DELAY_TO),
        }
    }

    pub fn mine_block(&mut self) {
        let mine_result = self.miner.mine();
        self.ctx.emit(
            BlockMined {
                block: mine_result.block,
            },
            self.ctx.id(),
            mine_result.delay,
        );
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl EventHandler for Node {
    fn on(&mut self, event: Event) {
        cast!(match event.data {
            BlockMined { .. } => {
                self.data.blocks_mined += 1;

                if self.ctx.time() < 10.0 {
                    self.mine_block();
                }
            }
        })
    }
}

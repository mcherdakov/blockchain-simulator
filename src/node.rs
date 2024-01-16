use crate::events::{BlockMined, Ping, Pong};
use crate::miner::Uniform;
use dslab_core::{cast, Event, EventHandler, Id, SimulationContext};

const MINE_DELAY_FROM: f64 = 0.1;
const MINE_DELAY_TO: f64 = 1.5;

#[derive(Default, Debug)]
pub struct Data {
    pub pings_sent: u32,
    pub pings_recieved: u32,
    pub pongs_sent: u32,
    pub pongs_recieved: u32,
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

    pub fn send_ping(&mut self, dst: Id) {
        self.data.pings_sent += 1;
        self.ctx.emit(Ping {}, dst, 1.0);
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
            Ping {} => {
                self.data.pings_recieved += 1;

                self.data.pongs_sent += 1;
                self.ctx.emit(Pong {}, event.src, 1.0);
            }
            Pong {} => {
                self.data.pongs_recieved += 1;

                if self.ctx.time() < 10.0 {
                    self.data.pings_sent += 1;
                    self.ctx.emit(Ping {}, event.src, 1.0);
                }
            }
            BlockMined { .. } => {
                self.data.blocks_mined += 1;

                if self.ctx.time() < 10.0 {
                    self.mine_block();
                }
            }
        })
    }
}

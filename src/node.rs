use crate::events::{Ping, Pong};
use dslab_core::{cast, Event, EventHandler, Id, SimulationContext};

#[derive(Debug)]
pub struct Data {
    pub pings_sent: u32,
    pub pings_recieved: u32,
    pub pongs_sent: u32,
    pub pongs_recieved: u32,
}

pub struct Node {
    ctx: SimulationContext,
    data: Data,
}

impl Node {
    pub fn new(ctx: SimulationContext) -> Self {
        Self {
            ctx,
            data: Data {
                pings_sent: 0,
                pings_recieved: 0,
                pongs_sent: 0,
                pongs_recieved: 0,
            },
        }
    }

    pub fn send_ping(&mut self, dst: Id) {
        self.data.pings_sent += 1;
        self.ctx.emit(Ping {}, dst, 1.0);
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

                if self.data.pings_sent < 10 {
                    self.data.pings_sent += 1;
                    self.ctx.emit(Ping {}, event.src, 1.0);
                }
            }
        })
    }
}

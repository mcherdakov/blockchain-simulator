use dslab_core::{
    event::{EventData, EventId},
    Id, SimulationContext,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct UniformNetwork {
    rng: ChaCha8Rng,
    delay_from: f64,
    delay_to: f64,
}

impl UniformNetwork {
    pub fn new(seed: u64, delay_from: f64, delay_to: f64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            delay_from,
            delay_to,
        }
    }

    pub fn send<T>(&mut self, ctx: &SimulationContext, data: T, dst: Id) -> EventId
    where
        T: EventData,
    {
        let delay = if ctx.id() == dst {
            0.0
        } else {
            self.rng.gen_range(self.delay_from..=self.delay_to)
        };

        ctx.emit(data, dst, delay)
    }
}

mod block;
mod block_storage;
mod config;
mod events;
mod miner;
mod network;
mod node;
mod node_network;
mod stats;

use dslab_core::Simulation;
pub use node::Node;
use node_network::NodeNetwork;
use std::error::Error;
pub use {config::Config, config::NodesConfig};

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut sim = Simulation::new(cfg.seed);

    let node_network = NodeNetwork::from_config(&cfg, &mut sim)?;

    for node in node_network.nodes().iter() {
        node.borrow_mut().start();
    }

    sim.steps(50);

    for node in node_network.nodes().iter() {
        let node = node.borrow();

        println!(
            "node {}:\nstats: {:?}\nstorage: {:?}",
            node.id(),
            node.stats(),
            node.storage(),
        );
    }

    Ok(())
}

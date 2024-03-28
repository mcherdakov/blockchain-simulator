use dslab_core::Simulation;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::block::Block;
use crate::config::{NodesConfig, RawNodes};
use crate::network::UniformNetwork;
use crate::node::Node;
use crate::Config;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

const NETWORK_DELAY_FROM: f64 = 1.0;
const NETWORK_DELAY_TO: f64 = 10.0;

#[derive(Debug, Clone)]
pub struct InvalidPeerError;

impl Display for InvalidPeerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid peers in provided config")
    }
}

impl Error for InvalidPeerError {}

pub struct NodeNetwork {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl NodeNetwork {
    pub fn from_config(cfg: &Config, sim: &mut Simulation) -> Result<Self, InvalidPeerError> {
        match &cfg.nodes_config {
            NodesConfig::Raw(raw) => NodeNetwork::from_raw_config(raw, sim, cfg.seed),
        }
    }

    pub fn from_raw_config(
        cfg: &RawNodes,
        sim: &mut Simulation,
        seed: u64,
    ) -> Result<Self, InvalidPeerError> {
        let mut nodes_by_name = HashMap::new();
        let mut nodes = Vec::new();

        let genesis_block = Block::genesis(&mut ChaCha8Rng::seed_from_u64(seed));

        let network = Rc::new(RefCell::new(UniformNetwork::new(
            seed,
            NETWORK_DELAY_FROM,
            NETWORK_DELAY_TO,
        )));

        for node_config in cfg.nodes.iter() {
            let node = create_node(
                sim,
                &node_config.name,
                seed,
                genesis_block.clone(),
                Rc::clone(&network),
            );

            nodes.push(node.clone());
            nodes_by_name.insert(node_config.name.clone(), node.clone());
        }

        for node_config in cfg.nodes.iter() {
            let node = nodes_by_name.get(&node_config.name).unwrap();

            for peer_name in node_config.peers.iter() {
                let mut peers = Vec::new();

                match nodes_by_name.get(peer_name) {
                    Some(peer_node) => {
                        peers.push(peer_node.borrow().id());
                    }
                    None => return Err(InvalidPeerError {}),
                }

                node.borrow_mut().add_peers(&peers);
            }
        }

        Ok(Self { nodes })
    }

    pub fn nodes(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.nodes
    }
}

fn create_node(
    sim: &mut Simulation,
    name: &str,
    seed: u64,
    genesis_block: Block,
    network: Rc<RefCell<UniformNetwork>>,
) -> Rc<RefCell<Node>> {
    let node = Rc::new(RefCell::new(Node::new(
        sim.create_context(name),
        seed,
        genesis_block,
        network,
    )));
    sim.add_handler(name, node.clone());

    node
}

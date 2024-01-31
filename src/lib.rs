mod block;
mod block_storage;
mod events;
mod miner;
mod node;

use dslab_core::Simulation;
use node::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Config {
    pub seed: u64,
}

fn create_node(sim: &mut Simulation, name: &str, seed: u64) -> (Rc<RefCell<Node>>, u32) {
    let node = Rc::new(RefCell::new(Node::new(sim.create_context(name), seed)));
    let node_id = sim.add_handler(name, node.clone());

    (node, node_id)
}

pub fn run(cfg: Config) {
    let mut sim = Simulation::new(cfg.seed);

    let (node1, _node1_id) = create_node(&mut sim, "node1", cfg.seed);
    let (node2, _node2_id) = create_node(&mut sim, "node2", cfg.seed);

    node1.borrow_mut().mine_block();
    node2.borrow_mut().mine_block();

    sim.step_until_no_events();

    println!(
        "node1:\n stats: {:?}\n storage: {:?}",
        node1.borrow().stats(),
        node1.borrow().storage()
    );
    println!(
        "node2:\n stats: {:?}\n storage: {:?}",
        node2.borrow().stats(),
        node2.borrow().storage()
    );
}

mod block;
mod events;
mod miner;
mod node;

use dslab_core::Simulation;
use node::Node;
use std::cell::RefCell;
use std::rc::Rc;

fn create_node(sim: &mut Simulation, name: &str, seed: u64) -> (Rc<RefCell<Node>>, u32) {
    let node = Rc::new(RefCell::new(Node::new(sim.create_context(name), seed)));
    let node_id = sim.add_handler(name, node.clone());

    (node, node_id)
}

fn main() {
    let seed = 42;

    let mut sim = Simulation::new(seed);

    let (node1, _node1_id) = create_node(&mut sim, "node1", seed);
    let (node2, _node2_id) = create_node(&mut sim, "node2", seed);

    node1.borrow_mut().mine_block();
    node2.borrow_mut().mine_block();

    sim.step_until_no_events();

    println!("node1: {:?}", node1.borrow().data());
    println!("node2: {:?}", node2.borrow().data());
}

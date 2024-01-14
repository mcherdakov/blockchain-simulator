mod events;
mod node;

use dslab_core::Simulation;
use node::Node;
use std::cell::RefCell;
use std::rc::Rc;

fn create_node(sim: &mut Simulation, name: &str) -> (Rc<RefCell<Node>>, u32) {
    let node = Rc::new(RefCell::new(Node::new(sim.create_context(name))));
    let node_id = sim.add_handler(name, node.clone());

    (node, node_id)
}

fn main() {
    let mut sim = Simulation::new(42);

    let (node1, _node1_id) = create_node(&mut sim, "node1");
    let (node2, node2_id) = create_node(&mut sim, "node2");

    node1.borrow_mut().send_ping(node2_id);

    sim.step_until_no_events();

    println!("node1: {:?}", node1.borrow().data());
    println!("node2: {:?}", node2.borrow().data());
}

mod block;
mod block_storage;
mod config;
mod events;
mod miner;
mod node;

pub use config::Config;
use dslab_core::Simulation;
use node::Node;
use std::cell::RefCell;
use std::rc::Rc;

fn create_node(sim: &mut Simulation, name: &str, seed: u64) -> Rc<RefCell<Node>> {
    let node = Rc::new(RefCell::new(Node::new(sim.create_context(name), seed)));
    sim.add_handler(name, node.clone());

    node
}

pub fn run(cfg: Config) {
    let mut sim = Simulation::new(cfg.seed);

    let nodes = &[
        create_node(&mut sim, "node1", cfg.seed),
        create_node(&mut sim, "node2", cfg.seed),
        create_node(&mut sim, "node3", cfg.seed),
        create_node(&mut sim, "node4", cfg.seed),
    ];

    nodes[0].borrow_mut().add_peers(&[nodes[1].borrow().id()]);
    nodes[1].borrow_mut().add_peers(&[nodes[2].borrow().id()]);
    nodes[2].borrow_mut().add_peers(&[nodes[3].borrow().id()]);
    nodes[3].borrow_mut().add_peers(&[nodes[0].borrow().id()]);

    for node in nodes.iter() {
        node.borrow_mut().start();
    }

    sim.steps(1000);

    for node in nodes.iter() {
        let node = node.borrow();

        println!(
            "node {}:\n stats: {:?}\n storage: {:?}",
            node.id(),
            node.stats(),
            node.storage().size(),
        );
    }
}

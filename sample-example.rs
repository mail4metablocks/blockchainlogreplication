use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct Node {
    id: u64,
    log: Arc<Mutex<Vec<u64>>>,
    neighbors: Vec<Node>,
}

impl Node {
    fn new(id: u64) -> Node {
        Node {
            id,
            log: Arc::new(Mutex::new(Vec::new())),
            neighbors: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, node: Node) {
        self.neighbors.push(node);
    }

    fn replicate_log(&self) {
        let log = self.log.lock().unwrap();
        for neighbor in &self.neighbors {
            neighbor.receive_log(log.clone());
        }
    }

    fn receive_log(&self, log: Arc<Mutex<Vec<u64>>>) {
        let mut local_log = self.log.lock().unwrap();
        for entry in log.lock().unwrap().iter() {
            if !local_log.contains(entry) {
                local_log.push(*entry);
            }
        }
    }
}

fn main() {
    let mut nodes = HashMap::new();

    // Create nodes
    for i in 0..5 {
        let node = Node::new(i);
        nodes.insert(i, node);
    }

    // Connect nodes
    nodes.get_mut(&0).unwrap().add_neighbor(nodes.get(&1).unwrap().clone());
    nodes.get_mut(&0).unwrap().add_neighbor(nodes.get(&2).unwrap().clone());
    nodes.get_mut(&1).unwrap().add_neighbor(nodes.get(&2).unwrap().clone());
    nodes.get_mut(&1).unwrap().add_neighbor(nodes.get(&3).unwrap().clone());
    nodes.get_mut(&2).unwrap().add_neighbor(nodes.get(&3).unwrap().clone());
    nodes.get_mut(&2).unwrap().add_neighbor(nodes.get(&4).unwrap().clone());
    nodes.get_mut(&3).unwrap().add_neighbor(nodes.get(&4).unwrap().clone());

    // Add entries to node 0's log
    let mut log = nodes.get(&0).unwrap().log.lock().unwrap();
    log.push(1);
    log.push(2);
    log.push(3);

    // Replicate log to neighbors
    nodes.get(&0).unwrap().replicate_log();

    // Check logs of all nodes
    for (id, node) in nodes.iter() {
        let log = node.log.lock().unwrap();
        println!("Node {} has log: {:?}", id, log);
    }
}

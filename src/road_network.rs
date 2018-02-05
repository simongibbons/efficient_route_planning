extern crate geo;

use geo_utils::Location;

use std::collections::HashMap;
use std::error::Error;

type NodeIndex = u64;
type Cost = u64;

pub struct Node {
    pub id: NodeIndex,
    pub neighbours: Vec<Edge>,
    pub location: Location,
}

impl Node {
    pub fn new(id: NodeIndex, location: Location) -> Self {
        Node {id, location, neighbours: Vec::new() }
    }
}

pub struct Edge {
    pub destination: NodeIndex,
    pub cost: Cost
}


pub struct RoadNetwork {
    nodes: HashMap<NodeIndex, Node>
}

impl RoadNetwork {

    /// Construct an empty network.
    pub fn new() -> Self {
        RoadNetwork {nodes: HashMap::new()}
    }


    /// Adds a node to the graph
    pub fn add_node(&mut self, node: Node) -> Result<(), Box<Error>> {
        if self.nodes.contains_key(&node.id) {
            return Err(From::from(format!("Attempted to add duplicate node {}", node.id)))
        }
        self.nodes.insert(node.id, node);
        Ok(())
    }


    /// Adds a directed edge to the graph
    ///
    /// TODO (Simon): Handle edges between nodes that don't exist already
    pub fn add_edge(&mut self, from_node: NodeIndex, to_node: NodeIndex, cost: Cost) {
        let from_node = self.get_node_mut(from_node).unwrap();
        from_node.neighbours.push(Edge {destination: to_node, cost})
    }


    /// Calculates the number of nodes in the graph.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }


    /// Calculates the number of edges in the graph.
    ///
    /// Warning O(n) in the number of vertices.
    pub fn num_edges(&self) -> usize {
        self.nodes.iter()
            .map(|(_, node)| node.neighbours.len())
            .sum()
    }


    pub fn get_node_mut(&mut self, node_id: NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(&node_id)
    }


    pub fn get_node(&self, node_id: NodeIndex) -> Option<&Node> {
        self.nodes.get(&node_id)
    }


    pub fn remove_unused_nodes(&mut self) {
        let nodes_to_remove: Vec<_> = self.nodes
            .iter()
            .filter(|&(_, ref node)| node.neighbours.is_empty())
            .map(|(k, _)| k.clone())
            .collect();

        for node_id in nodes_to_remove {
            self.nodes.remove(&node_id);
        }
    }
}


#[cfg(test)]
mod tests {
    use road_network::*;

    #[test]
    fn test_construct_network() {
        let mut network = RoadNetwork::new();

        network.add_node(Node::new(1, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(2, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(3, Location::new(0., 0.))).unwrap();

        assert_eq!(3, network.num_nodes());

        network.add_edge(1, 2, 10);
        network.add_edge(2, 1, 10);
        network.add_edge(3, 2, 10);
        network.add_edge(1, 3, 10);

        assert_eq!(4, network.num_edges());
    }
}

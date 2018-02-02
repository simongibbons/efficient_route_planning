use std::collections::HashMap;

type NodeIndex = u64;
type Cost = u64;

struct Node {
    pub neighbours: Vec<Edge>
}

impl Node {
    pub fn new() -> Self {
        Node { neighbours: Vec::new() }
    }
}


struct Edge {
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
    pub fn add_node(&mut self, node_id: NodeIndex) {
        self.nodes.insert(node_id, Node::new());
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


    fn get_node_mut(&mut self, node_id: NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(&node_id)
    }
}


#[cfg(test)]
mod tests {
    use road_network::*;

    #[test]
    fn test_construct_network() {
        let mut network = RoadNetwork::new();

        network.add_node(1);
        network.add_node(2);
        network.add_node(3);

        assert_eq!(3, network.num_nodes());

        network.add_edge(1, 2, 10);
        network.add_edge(2, 1, 10);
        network.add_edge(3, 2, 10);
        network.add_edge(1, 3, 10);

        assert_eq!(4, network.num_edges());
    }
}

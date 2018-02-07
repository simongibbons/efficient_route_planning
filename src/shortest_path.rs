use road_network::{Cost, RoadNetwork, Node, NodeIndex};

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;


#[derive(Debug, Default, Eq, PartialEq)]
struct HeapEl {
    cost: Cost,
    node_index: NodeIndex,
    previous_node_index: Option<NodeIndex>,
}


impl Ord for HeapEl {
    fn cmp(&self, other: &HeapEl) -> Ordering {
        // Flip the order of comparisons to turn the heap into a min-heap
        other.cost.cmp(&self.cost)
    }
}


impl PartialOrd for HeapEl {
    fn partial_cmp(&self, other: &HeapEl) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct ShortestPath {
    cost: Cost,
    path: Vec<NodeIndex>,
}


pub fn shortest_path(network: &RoadNetwork,
                     start_node: &Node,
                     end_node: &Node) -> Option<ShortestPath> {

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut previous_nodes = HashMap::new();

    heap.push(HeapEl {cost: 0, node_index: start_node.id, previous_node_index: None});

    while let Some(el) = heap.pop() {
        if visited.contains(&el.node_index) {
            continue;
        }
        visited.insert(el.node_index);
        previous_nodes.insert(el.node_index, el.previous_node_index);

        if el.node_index == end_node.id {
            return Some(ShortestPath {
                cost: el.cost,
                path: trace_path(previous_nodes, end_node.id)
            });
        }

        let node = network.get_node(el.node_index).unwrap();
        for neighbour in node.neighbours.iter() {
            heap.push(HeapEl {
                cost: el.cost + neighbour.cost,
                node_index: neighbour.destination,
                previous_node_index: Some(el.node_index),
            });
        }
    }

    None
}


fn trace_path(previous_nodes: HashMap<NodeIndex, Option<NodeIndex>>,
              end_node: NodeIndex) -> Vec<NodeIndex> {
    let mut path = Vec::new();
    let mut current_node = Some(end_node);

    // TODO (Simon): There should be a nicer way of writing this.
    while current_node.is_some() {
        path.push(current_node.unwrap());
        current_node = *previous_nodes.get(&current_node.unwrap()).unwrap();
    }

    path.reverse();
    path
}


#[cfg(test)]
mod tests {
    use shortest_path::*;

    use geo_utils::Location;

    fn get_test_network() -> RoadNetwork {
        let mut network = RoadNetwork::new();

        network.add_node(Node::new(1, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(2, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(3, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(4, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(5, Location::new(0., 0.))).unwrap();

        network.add_edge(1, 2, 5);
        network.add_edge(2, 3, 10);
        network.add_edge(3, 4, 20);
        network.add_edge(1, 4, 100);

        network
    }


    #[test]
    fn test_heap() {
        let mut heap = BinaryHeap::new();

        heap.push(HeapEl { node_index: 1, cost: 10, previous_node_index: None });
        heap.push(HeapEl { node_index: 2, cost: 0, previous_node_index: None } );
        heap.push(HeapEl { node_index: 3, cost: 100, previous_node_index: None });

        assert_eq!(3, heap.len());
        assert_eq!(2, heap.pop().unwrap().node_index);
        assert_eq!(1, heap.pop().unwrap().node_index);
        assert_eq!(3, heap.pop().unwrap().node_index);
        assert_eq!(None, heap.pop());

        assert_eq!(0, heap.len());
    }


    #[test]
    fn test_route_to_same_node() {
        let network = get_test_network();
        let node = network.get_node(1).unwrap();

        let result = shortest_path(&network, &node, &node).unwrap();
        assert_eq!(0, result.cost);
        assert_eq!(vec![1], result.path);
    }


    #[test]
    fn test_no_path_between_start_and_end() {
        let network = get_test_network();
        let start = network.get_node(5).unwrap();
        let end = network.get_node(1).unwrap();
        assert_eq!(None, shortest_path(&network, start, end));
    }


    #[test]
    fn test_path_shorter_when_traversing_more_vertices() {
        let network = get_test_network();
        let start = network.get_node(1).unwrap();
        let end = network.get_node(4).unwrap();

        let result = shortest_path(&network, start, end).unwrap();
        assert_eq!(35, result.cost);
        assert_eq!(vec![1, 2, 3, 4], result.path);
    }
}

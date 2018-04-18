use road_network::{RoadNetwork, NodeIndex};

use std::collections::HashSet;

pub type ConnectedComponent = Vec<NodeIndex>;

/// Use Kosaraju's algorithm to find strongly connected components of a road network
pub fn strongly_connected_components(network: &RoadNetwork) -> Vec<ConnectedComponent> {
    let nodes_in_order = nodes_in_order(network);
    assign_nodes_to_components(network, nodes_in_order)
}


fn nodes_in_order(network: &RoadNetwork) -> Vec<NodeIndex> {
    struct State<'a> {
        network: &'a RoadNetwork,
        visited: HashSet<NodeIndex>,
        result: Vec<NodeIndex>,
    };

    fn visit(node_index: NodeIndex, state: &mut State) {
        if state.visited.contains(&node_index) {
            return;
        }
        state.visited.insert(node_index);

        let node = state.network.get_node(node_index).unwrap();
        for neighbour in node.neighbours.iter() {
            visit(neighbour.destination, state)
        }
        state.result.push(node_index);
    }


    let mut state = State {
        network,
        visited: HashSet::with_capacity(network.num_nodes()),
        result: Vec::with_capacity(network.num_nodes()),
    };
    for (node_index, _) in network.nodes_iter() {
        visit(*node_index, &mut state);
    }

    state.result.reverse();
    return state.result;
}


fn assign_nodes_to_components(network: &RoadNetwork,
                              nodes_in_order: Vec<NodeIndex>) -> Vec<ConnectedComponent> {
    let mut assigned_nodes = HashSet::new();
    let mut result = Vec::new();
    for node_index in nodes_in_order.into_iter() {
        if assigned_nodes.contains(&node_index) {
            continue;
        }
        result.push(build_component(network, node_index, &mut assigned_nodes));
    }
    result
}


fn build_component(network: &RoadNetwork,
                   root: NodeIndex,
                   assigned_nodes: &mut HashSet<NodeIndex>) -> ConnectedComponent {

    let mut component = ConnectedComponent::new();
    if assigned_nodes.contains(&root) {
        return component;
    }

    let mut stack = vec![root];
    while stack.len() > 0 {
        let node_index = stack.pop().unwrap();
        assigned_nodes.insert(node_index);
        component.push(node_index);

        let node = network.get_node(node_index).unwrap();
        for reverse_neighbour in node.reverse_neighbours.iter() {
            if !assigned_nodes.contains(&reverse_neighbour.origin) {
                stack.push(reverse_neighbour.origin);
            }
        }
    }

    component.sort();
    component
}


#[cfg(test)]
mod tests {
    use connected_components::*;
    use road_network::{Node, RoadNetwork};
    use geo_utils::Location;


    fn build_triangle_network() -> RoadNetwork {
        let mut network = RoadNetwork::new();

        network.add_node(Node::new(0, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(1, Location::new(0., 0.))).unwrap();
        network.add_node(Node::new(2, Location::new(0., 0.))).unwrap();

        network.add_edge(0, 1, 10);
        network.add_edge(1, 2, 10);
        network.add_edge(2, 0, 10);

        network
    }

    fn build_network_with_weakly_connected_node() -> RoadNetwork {
        let mut network = build_triangle_network();
        network.add_node(Node::new(99, Location::new(0., 0.))).unwrap();
        network.add_edge(0, 99, 10);

        network
    }

    #[test]
    fn test_network_with_one_component() {
        let network = build_triangle_network();
        let components = strongly_connected_components(&network);

        assert_eq!(1, components.len());
        assert_eq!(vec![0, 1, 2], components[0]);
    }

    #[test]
    fn test_network_with_weakly_connected_node() {
        let network = build_network_with_weakly_connected_node();
        let components = strongly_connected_components(&network);

        assert_eq!(2, components.len());
        assert!(components.contains(&vec![99]));
        assert!(components.contains(&vec![0, 1, 2]));
    }
}

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

    fn assign(node_index: NodeIndex,
              network: &RoadNetwork,
              assigned_nodes: &mut HashSet<NodeIndex>,
              component: &mut ConnectedComponent) {

        if assigned_nodes.contains(&node_index) {
            return;
        }
        assigned_nodes.insert(node_index);
        component.push(node_index);

        let node = network.get_node(node_index).unwrap();
        for reverse_neighbour in node.reverse_neighbours.iter() {
            assign(reverse_neighbour.origin, network, assigned_nodes, component);
        }
    }

    assign(root, network, assigned_nodes, &mut component);
    component
}

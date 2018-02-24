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


/// TODO(Simon): Finish this function (see part 3 of https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm)
fn assign_nodes_to_components(network: &RoadNetwork,
                              nodes_in_order: Vec<NodeIndex>) -> Vec<ConnectedComponent> {
    
}

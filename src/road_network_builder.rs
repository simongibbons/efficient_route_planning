extern crate itertools;

use itertools::Itertools;

use geo_utils::{earth_distance, Location};
use road_network::{RoadNetwork, Node};
use osm_reader::{Osm, OsmNode, OsmWay};

use std::error::Error;


pub fn build_road_network_from_osm(osm: Osm) -> Result<RoadNetwork, Box<Error>> {
    let mut network = RoadNetwork::new();
    network = add_nodes_to_network(network, &osm.nodes)?;
    network = add_ways_to_network(network, &osm.ways)?;
    network.reduce_to_largest_strongly_connected_component();
    Ok(network)
}


fn add_nodes_to_network(mut network: RoadNetwork,
                        nodes: &Vec<OsmNode>) -> Result<RoadNetwork, Box<Error>>  {
    for osm_node in nodes.iter() {
        let location = Location::new(osm_node.lat, osm_node.lon);
        network.add_node(Node::new(osm_node.id, location))?;
    }
    Ok(network)
}


fn add_ways_to_network(mut network: RoadNetwork,
                       ways: &Vec<OsmWay>) -> Result<RoadNetwork, Box<Error>> {
    for way in ways.iter() {
        network = add_way_to_network(network, &way)?;
    }
    Ok(network)
}


fn add_way_to_network(mut network: RoadNetwork, way: &OsmWay) -> Result<RoadNetwork, Box<Error>> {
    let highway_type = way.highway_type();
    if highway_type.is_none() {
        return Ok(network);
    }
    let highway_type = highway_type.unwrap();

    // TODO (Simon): Compute edge costs correctly here.
    for (start_nd, end_nd) in way.nodes.iter().tuple_windows() {
        let cost = edge_duration_seconds(&network, start_nd.ref_, end_nd.ref_, highway_type.speed_ms());
        network.add_edge(start_nd.ref_, end_nd.ref_, cost);
        network.add_edge(end_nd.ref_, start_nd.ref_, cost);
    }

    Ok(network)
}


fn edge_duration_seconds(network: &RoadNetwork, start_id: u64, end_id: u64, speed_ms: f64) -> u64 {
    let start_node = network.get_node(start_id).unwrap();
    let end_node = network.get_node(end_id).unwrap();

    let distance_meters = earth_distance(&start_node.location, &end_node.location);
    (distance_meters / speed_ms) as u64
}

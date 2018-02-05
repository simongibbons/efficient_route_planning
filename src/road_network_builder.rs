extern crate geo;
extern crate itertools;

use self::geo::Point;

use itertools::Itertools;

use road_network::{RoadNetwork, Node};
use osm_reader::{Osm, OsmNode, OsmWay};

use std::error::Error;


pub fn build_road_network_from_osm(osm: Osm) -> Result<RoadNetwork, Box<Error>> {
    let mut network = RoadNetwork::new();
    network = add_nodes_to_network(network, &osm.nodes)?;
    network = add_ways_to_network(network, &osm.ways)?;
    Ok(network)
}


fn add_nodes_to_network(mut network: RoadNetwork,
                        nodes: &Vec<OsmNode>) -> Result<RoadNetwork, Box<Error>>  {
    for osm_node in nodes.iter() {
        let point = Point::new(osm_node.lon, osm_node.lat);
        network.add_node(Node::new(osm_node.id, point))?;
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

    // TODO (Simon): Compute edge costs correctly here.
    for (start_nd, end_nd) in way.nodes.iter().tuple_windows() {
        network.add_edge(start_nd.ref_, end_nd.ref_, 1);
    }

    if !way.is_oneway() {
        for (start_nd, end_nd) in way.nodes.iter().rev().tuple_windows() {
            network.add_edge(start_nd.ref_, end_nd.ref_, 1);
        }
    }

    Ok(network)
}

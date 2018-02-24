extern crate efficient_route_planning;

use efficient_route_planning::connected_components::strongly_connected_components;
use efficient_route_planning::osm_reader::read_osm_extract;
use efficient_route_planning::road_network_builder::build_road_network_from_osm;


fn main() {
    println!("Reading Extract");
    let osm = read_osm_extract("saarland.osm").unwrap();

    println!("Constructing Graph");
    let road_network = build_road_network_from_osm(osm).unwrap();

    strongly_connected_components(&road_network);

    println!("Num Nodes: {}", road_network.num_nodes());
    println!("Num Edges: {}", road_network.num_edges());
    println!("Num Strongly Connected Components: {}", strongly_connected_components(&road_network).len());
}

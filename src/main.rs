extern crate efficient_route_planning;

use efficient_route_planning::osm_reader::read_osm_extract;
use efficient_route_planning::road_network_builder::build_road_network_from_osm;


fn main() {
    println!("Reading Extract");
    let osm = read_osm_extract("saarland.osm").unwrap();

    println!("Constructing Graph");
    let road_network = build_road_network_from_osm(osm).unwrap();

    println!("{}", road_network.num_nodes());
    println!("{}", road_network.num_edges());
}

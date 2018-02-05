extern crate geo;

use self::geo::Point;
use self::geo::algorithm::haversine_distance::HaversineDistance;


#[derive(Debug, PartialEq)]
pub struct Location {
    point: Point<f64>,
}


impl Location {
    pub fn new(lat: f64, lng: f64) -> Self {
        Location {point: Point::new(lng, lat)}
    }

    pub fn as_point(&self) -> &Point<f64> {
        &self.point
    }
}


pub fn earth_distance(l1: &Location, l2: &Location) -> f64 {
    l1.as_point().haversine_distance(l2.as_point())
}

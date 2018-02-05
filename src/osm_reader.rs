extern crate serde;
extern crate serde_xml_rs;

use self::serde::{de, Deserialize, Deserializer};
use self::serde_xml_rs::deserialize;

use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;


#[derive(Debug, Eq, PartialEq)]
pub enum HighwayType {
    Motorway,
    Trunk,
    Primary,
    Secondary,
    Tertiary,
    MotorwayLink,
    TrunkLink,
    PrimaryLink,
    SecondaryLink,
    TertiaryLink,
    Road,
    Unclassified,
    Residential,
    Unsurfaced,
    LivingStreet,
    Service,
}


impl HighwayType {

    pub fn from_str(s: &str) -> Option<Self> {
        use self::HighwayType::*;
        match s {
            "motorway" => Some(Motorway),
            "trunk" => Some(Trunk),
            "primary" => Some(Primary),
            "secondary" => Some(Secondary),
            "tertiary" => Some(Tertiary),
            "motorway_link" => Some(MotorwayLink),
            "trunk_link" => Some(TrunkLink),
            "primary_link" => Some(PrimaryLink),
            "secondary_link" => Some(SecondaryLink),
            "tertiary_link" => Some(TertiaryLink),
            "road" => Some(Road),
            "unclassified" => Some(Unclassified),
            "residential" => Some(Residential),
            "unsurfaced" => Some(Unsurfaced),
            "living_street" => Some(LivingStreet),
            "service" => Some(Service),
            _ => None
        }
    }

    pub fn speed_ms(&self) -> f64 {
        use self::HighwayType::*;
        let speed_kmh = match *self {
            Motorway => 110.0,
            Trunk => 110.0,
            Primary => 70.0,
            Secondary => 60.0,
            Tertiary => 50.0,
            MotorwayLink => 50.0,
            TrunkLink => 50.0,
            PrimaryLink => 50.0,
            SecondaryLink => 50.0,
            TertiaryLink => 50.0,
            Road => 40.0,
            Unclassified => 40.0,
            Residential => 30.0,
            Unsurfaced => 30.0,
            LivingStreet => 10.0,
            Service => 10.0,
        };
        kmh_to_ms(speed_kmh)
    }
}

fn kmh_to_ms(speed_in_kmh: f64) -> f64 {
    speed_in_kmh / 3.6
}


#[derive(Debug, Deserialize)]
pub struct OsmNd {
    #[serde(deserialize_with = "de_u64_from_str")]
    #[serde(rename = "ref", default)]
    pub ref_: u64,
}


#[derive(Debug, Deserialize)]
pub struct OsmTag {
    #[serde(rename = "k", default)]
    key: String,
    #[serde(rename = "v", default)]
    value: String,
}


#[derive(Debug, Deserialize)]
pub struct OsmNode {
    #[serde(deserialize_with = "de_u64_from_str")]
    pub id: u64,
    #[serde(deserialize_with = "de_f64_from_str")]
    pub lat: f64,
    #[serde(deserialize_with = "de_f64_from_str")]
    pub lon: f64,
}


#[derive(Debug, Deserialize)]
pub struct OsmWay {
    #[serde(deserialize_with = "de_u64_from_str")]
    id: u64,

    #[serde(rename = "nd", default)]
    pub nodes: Vec<OsmNd>,

    #[serde(rename = "tag", default)]
    tags: Vec<OsmTag>,
}


impl OsmWay {

    pub fn highway_type(&self) -> Option<HighwayType> {
        self.get_tag_value("highway")
            .and_then(|tag_value| HighwayType::from_str(&tag_value))
    }

    pub fn is_oneway(&self) -> bool {
        self.get_tag_value("oneway")
            .map_or(false, |value| value == "yes")
    }

    fn get_tag_value(&self, key: &str) -> Option<&str> {
        for tag in self.tags.iter() {
            if tag.key == key {
                return Some(&tag.value);
            }
        }
        None
    }
}


#[derive(Debug, Deserialize)]
pub struct Osm {
    #[serde(rename = "node", default)]
    pub nodes: Vec<OsmNode>,
    #[serde(rename = "way", default)]
    pub ways: Vec<OsmWay>,
}


pub fn read_osm_extract(file_name: &str) -> Result<Osm, Box<::std::error::Error>>  {
    let f = File::open(file_name)?;
    let reader = BufReader::new(&f);
    match deserialize(reader) {
        Ok(osm) => Ok(osm),
        Err(e) => Err(From::from(format!("{:?}", e)))
    }
}


// TODO (Simon): Find a way to make these generic.
fn de_u64_from_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where D: Deserializer<'de>
{
    let s = <String>::deserialize(deserializer)?;
    u64::from_str(&s).map_err(de::Error::custom)
}


fn de_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = <String>::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}


#[cfg(test)]
mod tests {
    use osm_reader::*;

    use std::path::Path;

    #[test]
    fn test_read_node() {
        let s = r##"
	    <node id="470558" lat="49.3414269" lon="7.3000691"/>
        "##;

        let osm_node: OsmNode = deserialize(s.as_bytes()).unwrap();
        assert_eq!(470558, osm_node.id);
    }

    #[test]
    fn test_read_way() {
        let s = r##"
        <way id="26659127" visible="true">
            <nd ref="292403538"/>
            <nd ref="298884289"/>
            <nd ref="261728686"/>
            <tag k="name" v="Pastower Straße"/>
            <tag k="highway" v="unclassified"/>
        </way>
        "##;

        let osm_way: OsmWay = deserialize(s.as_bytes()).unwrap();
        assert_eq!(26659127, osm_way.id);

        assert_eq!(3, osm_way.nodes.len());
        assert_eq!(292403538, osm_way.nodes[0].ref_);

        assert_eq!(2, osm_way.tags.len());
        let tag = &osm_way.tags[0];
        assert_eq!("name", tag.key);
        assert_eq!("Pastower Straße", tag.value);

        assert_eq!(HighwayType::Unclassified, osm_way.highway_type().unwrap());
        assert_eq!(false, osm_way.is_oneway());
    }

    #[test]
    fn test_read_osm() {
        let s = r##"
        <?xml version='1.0' encoding='UTF-8'?>
        <osm version="0.6" generator="pbf2osm">
	        <node id="470552" lat="49.3413853" lon="7.3014897"/>
	        <node id="470553" lat="49.3407084" lon="7.3006280"/>
	        <way id="26659127" visible="true">
	            <nd ref="470552"/>
                <nd ref="470553"/>
            </way>
        </osm>
        "##;

        let osm: Osm = deserialize(s.as_bytes()).unwrap();
        assert_eq!(2, osm.nodes.len());
        assert_eq!(1, osm.ways.len());
    }

    #[test]
    fn test_read_osm_extract_file() {
        let fixture_path = Path::new("tests").join("fixtures").join("test.osm");
        let osm = read_osm_extract(fixture_path.to_str().unwrap()).unwrap();
        assert_eq!(2, osm.nodes.len());
        assert_eq!(1, osm.ways.len());
    }

    #[test]
    fn test_missing_file() {
        let osm = read_osm_extract("not a file path");
        assert!(osm.is_err());
    }
}

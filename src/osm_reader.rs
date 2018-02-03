extern crate serde;
extern crate serde_xml_rs;

use self::serde::{de, Deserialize, Deserializer};
use self::serde_xml_rs::deserialize;

use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

use std::path::Path;


#[derive(Debug, Deserialize)]
pub struct OsmNd {
    #[serde(deserialize_with = "de_u64_from_str")]
    #[serde(rename = "ref", default)]
    ref_: u64,
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
    id: u64,
    #[serde(deserialize_with = "de_f64_from_str")]
    lat: f64,
    #[serde(deserialize_with = "de_f64_from_str")]
    lon: f64,
}


#[derive(Debug, Deserialize)]
pub struct OsmWay {
    #[serde(deserialize_with = "de_u64_from_str")]
    id: u64,

    #[serde(rename = "nd", default)]
    nodes: Vec<OsmNd>,

    #[serde(rename = "tag", default)]
    tags: Vec<OsmTag>,
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
    fn test_read_osm_extract() {
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

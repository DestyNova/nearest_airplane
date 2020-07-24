use serde::Deserialize;
use serde_json::from_reader;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Point {
    lat: f64,
    lon: f64,
}

impl Point {
    pub fn from_coords(s: &str) -> Self {
        let coords: Vec<f64> = s.split("\n").take(2).map(Point::parse_coord).collect();

        let lat = coords[0];
        let lon = coords[1];

        Point { lat, lon }
    }

    fn parse_coord(s: &str) -> f64 {
        let tokens: Vec<&str> = s.split(" ").collect();

        let sign = if tokens[1] == "S" || tokens[1] == "W" {
            -1.0
        } else {
            1.0
        };
        tokens[0]
            .to_string()
            .parse::<f64>()
            .expect(&format!("Float parse failure on {}", tokens[0]))
            * sign
    }
}

#[derive(Deserialize)]
struct OpenskyResponse {
    states: Vec<OpenskyState>,
}

#[derive(Debug, Deserialize)]
struct OpenskyState {
    icao24: String,
    callsign: String,
    origin_country: String,
    time_position: Option<usize>,
    last_contact: usize,
    longitude: Option<f64>,
    latitude: Option<f64>,
    baro_altitude: Option<f64>,
    on_ground: bool,
    velocity: Option<f64>,
    true_track: f64,
    vertical_rate: Option<f64>,
    sensors: Option<Vec<usize>>,
    geo_altitude: Option<f64>,
    squawk: Option<String>,
    spi: bool,
    position_source: usize,
}

fn main() {
    // read coords from stdin
    let mut coords = String::new();
    io::stdin()
        .read_to_string(&mut coords)
        .expect("Failed to read input coords.");
    let p = Point::from_coords(&coords);

    // call Opensky API and parse states
    let states = get_opensky_states();

    // calculate distances to each plane
    let mut results = states
        .iter()
        .flat_map(|state| match (state.latitude, state.longitude) {
            (Some(lat), Some(lon)) => {
                let plane_pos = Point { lat, lon };
                Some((haversine(&p, plane_pos), state))
            }
            _ => None,
        })
        .collect::<Vec<(f64, &OpenskyState)>>();

    // sort results by distance from the requested point
    results.sort_unstable_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());

    // take the closest one and tell us about it
    eprintln!("Plane states with known coordinates: {}", results.len());
    eprintln!(
        "Result: {:?} with distance {} km.",
        results[0].1, results[0].0
    );
}

fn get_opensky_states() -> Vec<OpenskyState> {
    match attohttpc::get("https://opensky-network.org/api/states/all").send() {
        Err(e) => panic!("Error calling Opensky API: {}", e),
        Ok(resp) => {
            let data = resp.bytes().expect("Error reading from Opensky API.");
            parse_opensky_response(data).states
        }
    }
}

fn parse_opensky_response(data: Vec<u8>) -> OpenskyResponse {
    from_reader(&data[..]).unwrap()
}

// Haversine formula implementation adapted from
// https://rosettacode.org/wiki/Haversine_formula#Rust
fn haversine(origin: &Point, destination: Point) -> f64 {
    const R: f64 = 6372.8;

    let o_lon = (origin.lon - destination.lon).to_radians();
    let o_lat = origin.lat.to_radians();
    let d_lat = destination.lat.to_radians();

    let dz: f64 = o_lat.sin() - d_lat.sin();
    let dx: f64 = o_lon.cos() * o_lat.cos() - d_lat.cos();
    let dy: f64 = o_lon.sin() * o_lat.cos();

    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::{fs::File, io::Read};

    fn read_file_bytes(path: &str) -> Vec<u8> {
        let mut f =
            File::open(path).expect(&format!("Can't open sample file {}", String::from(path)));
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .expect(&format!("Can't read sample file {}", String::from(path)));
        buf
    }

    #[test]
    fn test_parse_opensky_response() {
        let data = read_file_bytes("test/opensky_states_all.json");
        let states = parse_opensky_response(data).states;
        assert_eq!(states.len(), 4969);
        assert_eq!(states[0].squawk, Some("1571".to_string()));
    }

    #[test]
    fn test_haversine() {
        let origin: Point = Point {
            lat: 36.12,
            lon: -86.67,
        };
        let destination: Point = Point {
            lat: 33.94,
            lon: -118.4,
        };

        assert!((haversine(&origin, destination) - 2887.2599506071106).powi(2) < 0.00001);
    }

    #[test]
    fn test_parse_point() {
        let p: Point = Point {
            lat: 12.5,
            lon: -14.75,
        };

        let coords = "12.5 N\n14.75 W";

        assert_eq!(p, Point::from_coords(coords));
    }
}

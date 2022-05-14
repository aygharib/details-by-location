// Will accept a location and display details for the location

use std::env;
use std::collections::HashMap;

struct Location {
    name: String,
    temperature: f32,
    latitude: f32,
    longitude: f32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let location_name: &str = &args[1];

    let locations_map: HashMap<String, Location> = build_map();

    match locations_map.get(&String::from(location_name.to_lowercase())) {
        Some(location) => println!("Location: {} \nLatitude: {} \nLongitude: {} \nTemperature: {}", location.name, location.latitude, location.longitude, location.temperature),
        _ => println!("There is no information stored for this location"),
    }
}

fn build_map() -> HashMap<String, Location> {
    let mut locations_map: HashMap<String, Location> = HashMap::new();

    locations_map.insert(String::from("Toronto".to_lowercase()), Location {
        name: String::from("Toronto"),
        temperature: 23.3,
        latitude: 43.6532,
        longitude: 79.3832,
    });

    locations_map
}
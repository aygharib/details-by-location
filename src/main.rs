// CLI APP
// Will accept a location (Toronto) and display the temperature of the location

// Option 1:
// Store temperatures for each location in a hashmap
// Update this hashmap based on live data every X time
// If the user requests for a location that isn't listed, alert them of invalid input

// Option 2:
// Store a list of Location structs, one for each location. Locations have temp, latitude, longitude, season, etc.
// Pro: more data
// Con: how will i find the correct ele of array based on user input without going through the whole list?

// Option 3: <--- Doing this
// Map of Location structs. Not using a list since order doesn't matter
// Key: location name
// Value: Location struct


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

    // Create map to store data
    build_map();

}

fn build_map() {
    let mut locations_map: HashMap<String, Location> = HashMap::new();

    locations_map.insert(String::from("Toronto"), Location {
        name: String::from("Toronto"),
        temperature: 23.3,
        latitude: 43.6532,
        longitude: 79.3832,
    });

    let a = locations_map.get(&String::from("Toronto"));
    
    match locations_map.get(&String::from("Toronto")) {
        Some(lol) => println!("{}", lol.temperature),
        None => println!("Nothing"),
    }
}
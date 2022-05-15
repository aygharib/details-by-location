// Will accept a location and display details for the location

use std::env;
use std::collections::HashMap;

use dotenv::dotenv;
use reqwest::Error;

use serde::{Deserialize, Serialize};

struct Location {
    name: String,
    temperature: f32,
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize, Deserialize)]
struct Geocode {
    name: String,
    lat: f32,
    lon: f32,
    country: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
struct Temperature {
    name: String,
    main: Main,
}

#[derive(Serialize, Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
}

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let location_name: &str = &args[1];

    api_call_coords(String::from(location_name));

    // let locations_map: HashMap<String, Location> = build_map();

    // match locations_map.get(&String::from(location_name.to_lowercase())) {
    //     Some(location) => println!("Location: {} \nLatitude: {} \nLongitude: {} \nTemperature: {}", location.name, location.latitude, location.longitude, location.temperature),
    //     _ => println!("There is no information stored for this location"),
    // }
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

fn api_call_coords(location_name: String) -> Result<(), Error> {
    let key = env::var("API_KEY").expect("API_KEY doesn't exist in .env file");
    let mut coords_request_url = String::from("http://api.openweathermap.org/geo/1.0/direct?q=");
    coords_request_url.push_str(&location_name);
    coords_request_url.push_str("&limit=1&");
    coords_request_url.push_str("&appid=");
    coords_request_url.push_str(&key);
    println!("URL: {}", coords_request_url);

    let response: Vec<Geocode> = reqwest::blocking::get(coords_request_url)?.json()?;

    let mut temperature_request_url = String::from("https://api.openweathermap.org/data/2.5/weather?lat=");
    temperature_request_url.push_str(&response[0].lat.to_string());
    temperature_request_url.push_str("&lon=");
    temperature_request_url.push_str(&response[0].lon.to_string());
    temperature_request_url.push_str("&appid=");
    temperature_request_url.push_str(&key);
    println!("URL: {}", temperature_request_url);

    let response2: Temperature = reqwest::blocking::get(temperature_request_url)?.json()?;

    println!("City: {} \nState: {} \nCountry: {} \nLatitude: {} \nLongitude: {} \nTemperature: {}", response[0].name, response[0].state, response[0].country, response[0].lat, response[0].lon, response2.main.temp);

    Ok(())
}

fn api_call_temperature() {
    let temperature_request_url = "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}";
}
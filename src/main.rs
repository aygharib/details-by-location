use std::env;

use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};

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
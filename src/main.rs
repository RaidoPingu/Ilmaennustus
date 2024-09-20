use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    properties: Properties,
}
#[derive(Deserialize, Debug)]
struct Properties {
    timeseries: Vec<TimeSeries>,
}
#[derive(Deserialize, Debug)]
struct TimeSeries {
    time: String,
    data: Data,
}
#[derive(Deserialize, Debug)]
struct Data {
    instant: InstantData,
}
#[derive(Deserialize, Debug)]
struct InstantData {
    details: Details,
}
#[derive(Deserialize, Debug)]
struct Details {
    air_temperature: f64,
}

fn fetch_weather(lat: &str, lon: &str) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
        lat, lon
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Ilmaennustus/1.0")
        .send()?;

    let weather_response: WeatherResponse = response.json()?;
    Ok(weather_response)
}

fn main() -> Result<(), Box<dyn Error>> {
    let city_lat = "59.437";
    let city_lon = "24.7536";
    let weather = fetch_weather(city_lat, city_lon)?;

    for timeseries in weather.properties.timeseries {
        let mut counter = 0;
        let time = timeseries.time;
        let temperature = timeseries.data.instant.details.air_temperature;
        println!("Time: {}, Temperature: {}°C", time, temperature);
        counter = counter + 1;
        if counter > 7 {
            break;
        }
    }

    Ok(())
}

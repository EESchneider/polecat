extern crate reqwest;
extern crate regex;

use regex::Regex;
use std::env::args;
use std::io::Read;
use std::time::Duration;
use reqwest::header::{Headers, RetryAfter};

fn query_coords(client: &reqwest::Client) -> (f32, f32) {
    let geolocation_url = "http://api.geoiplookup.net";
    let geo_res = client.get(geolocation_url).send();
    if geo_res.is_err() {
        println!("GOEIP FAILED");
    }
    let mut geo_res = geo_res.unwrap();
    let mut geo_text = String::new();
    geo_res.read_to_string(&mut geo_text).expect("Failed to read GeoIP");

    let lat: f32 = Regex::new(r"<latitude>(.*)</latitude>").unwrap().captures(&geo_text)
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
        .and_then(|x| x.parse().ok())
        .unwrap();
    let lon: f32 = Regex::new(r"<longitude>(.*)</longitude>").unwrap().captures(&geo_text)
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
        .and_then(|x| x.parse().ok())
        .unwrap();

    return (lat, lon);
}

fn main() {
    let mut client = reqwest::ClientBuilder::new();
    let mut retry = Headers::new();
    retry.set(RetryAfter::Delay(Duration::from_secs(20)));
    client.default_headers(retry);
    let client = client.build().unwrap();

    let appid = "886705b4c1182eb1c69f28eb8c520e20";
    let mut a = args();
    a.next();
    let weather_api_url = match a.next() {
        Some(x) => format!("http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&mode=json&appid={}", x.trim(), appid),
        None => format!("http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&mode=json&appid={}", query_coords(&client).0, query_coords(&client).1, appid)
    };
    let weather_res = client.get(&weather_api_url).send();
    if weather_res.is_err() {
        println!("WEATHER FAILED");
        return ();
    }
    let mut weather_res = weather_res.unwrap();

    let mut text = String::new();
    weather_res.read_to_string(&mut text).expect("Failed to read response");

    let temp: f32 = Regex::new(r#""temp":([^,]*)"#).unwrap().captures(&text)
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
        .and_then(|x| x.parse().ok()).expect("Failed to read temperature");

    let city = Regex::new(r#""name":"([^"]*)"#).unwrap().captures(&text)
                          .and_then(|x| x.get(1))
                          .map(|x| x.as_str())
                          .expect("Failed to read city name");

    let lat: f32 = Regex::new(r#""lat":([^,}]*)"#).unwrap().captures(&text)
                          .and_then(|x| x.get(1))
                          .map(|x| x.as_str()).unwrap()
                          .parse().expect("Failed to read latitude");
    let lon: f32 = Regex::new(r#""lon":([^,}]*)"#).unwrap().captures(&text)
                          .and_then(|x| x.get(1))
                          .map(|x| x.as_str()).unwrap()
                          .parse().expect("Failed to read longitude");

    let conditions = vec![
        ("", Regex::new(r#""id":2\d{2}"#).unwrap()), // Thunderstorm
        ("", Regex::new(r#""id":6\d{2}"#).unwrap()), // Snow
        ("", Regex::new(r#""id":5\d{2}"#).unwrap()), // Rain
        ("", Regex::new(r#""id":3\d{2}"#).unwrap()), // Drizzle
        ("", Regex::new(r#""id":7\d{2}"#).unwrap()), // Atmospheric
        ("", Regex::new(r#""id":8\d{2}"#).unwrap()), // Clouds
    ];

    let mut icon = conditions.iter().fold(
        ("", None),
        |acc, x| match acc {
            (_, None) => (x.0, x.1.find(&text)),
            found => found
        }
    ).0;

    if icon == "" {
        icon = "";
    }

    println!("{} {:.0} °C", icon, temp);
    println!("Results for: city={} lat={} lon={}", city, lat, lon);
}

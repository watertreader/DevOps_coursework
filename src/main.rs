// src/main.rs

use actix_web::{web, App, HttpServer, HttpResponse};
use reqwest::Client;
use serde::Deserialize;
use std::env;
use chrono::{DateTime, Utc, Local};
use geolocation;

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    name: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
}

async fn index() -> HttpResponse {
    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect(".env");
    let city = "Singapore";

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = Client::new().get(&url).send().await;

    match response {
        Ok(res) => {
            let weather_data: WeatherResponse = res.json().await.expect("Failed to parse JSON response");

            let temperature = weather_data.main.temp;
            let city_name = weather_data.name;
            let utc_time: DateTime<Utc> = Utc::now();
            let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
     
            HttpResponse::Ok().body(format!(" Today is {} \n Weather in {}: {:.1}°C", local_time, city_name, temperature))
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch weather data"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("0.0.0.0:4040")?
    .run()
    .await
}
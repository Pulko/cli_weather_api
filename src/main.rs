use colored::*;
use serde::{Deserialize, Serialize};
use std::{env::VarError, io};

const API_NAME_KEY: &str = "API_KEY";

#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?.json::<WeatherResponse>()?;
    Ok(response)
}

fn print_weather_info(weather_info: WeatherResponse) {
    let description = weather_info.weather[0].description.clone();

    println!(
        "\n\n{}\n",
        weather_info.name.to_uppercase().bright_white().bold()
    );
    println!(
        "> Weather: {}",
        get_description_emoji_and_color(description)
    );
    println!("> Temperature: {}", get_temp_emoji(weather_info.main.temp));
    println!(
        "> Pressure: {} hPa",
        weather_info.main.pressure.to_string().green().bold()
    );
    println!(
        "> Humidity: {}%",
        weather_info.main.humidity.to_string().green().bold()
    );
    println!(
        "> Wind speed: {} m/s",
        weather_info.wind.speed.to_string().green().bold()
    );
    println!("\n");
}

fn get_temp_emoji(temp: f64) -> ColoredString {
    if temp < 0.0 {
        return format!("{}°C 🫢", temp).cyan();
    } else if temp < 10.0 {
        return format!("{}°C 🥶", temp).blue();
    } else if temp < 20.0 {
        return format!("{}°C 😊", temp).bright_green();
    } else if temp < 30.0 {
        return format!("{}°C 🌞", temp).yellow();
    } else {
        return format!("{}°C 🔥", temp).red();
    }
}

fn get_description_emoji_and_color(description: String) -> ColoredString {
    match description.as_str() {
        "clear sky" => format!("{} 🌄", description).bright_yellow(),
        "few clouds" => format!("{} 🌤️", description).bright_blue(),
        "overcast clouds" => format!("{} 🌤️", description).bright_blue(),
        "scattered clouds" => format!("{} 🌥️", description).bright_blue(),
        "broken clouds" => format!("{} 🌫️", description).bright_blue(),
        "shower rain" => format!("{} 🌧️", description).bright_cyan(),
        "light rain" => format!("{} 🌧️", description).bright_cyan(),
        "light snow" => format!("{} 🌨️", description).bright_cyan(),
        "rain" => format!("{} 🌧️", description).bright_cyan(),
        "thunderstorm" => format!("{} ⛈️", description).bright_cyan(),
        "snow" => format!("{} 🌨️", description).bright_cyan(),
        "mist" => format!("{} 🌫️", description).dimmed(),
        _ => description.normal(),
    }
}

fn get_city_name() -> String {
    let mut city_name = String::new();
    while city_name.is_empty() {
        let q = format!("{}", String::from("Enter city name: ").white());
        println!("{}", q);
        io::stdin().read_line(&mut city_name).unwrap();
        city_name = city_name.trim().to_string();
    }

    city_name
}

fn get_country_code() -> String {
    let mut country_code = String::new();

    while country_code.is_empty() {
        let q = format!("{}", String::from("Enter country code: ").white());
        println!("{}", q);
        io::stdin().read_line(&mut country_code).unwrap();
        country_code = country_code.trim().to_string();
    }

    country_code
}

fn is_repeat() -> bool {
    println!("Do you want to get weather info for another city? (y/n)");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice = choice.trim().to_string();

    return choice.eq("y");
}

fn get_api_key() -> Result<String, VarError> {
    dotenv::dotenv().ok();

    let env_api_key = std::env::var(API_NAME_KEY);

    match env_api_key {
        Ok(key) => return Ok(key),
        Err(e) => {
            let err_message = format!(
                "{}: {} is not set in .env file. Visit openweathermap.org to get an API key.",
                e.to_string().to_uppercase(),
                API_NAME_KEY,
            )
            .red();
            println!("{}", err_message);

            return Err(VarError::NotPresent);
        }
    }
}

fn main() -> () {
    let api_key = get_api_key().unwrap_or("".to_string());
    if api_key.is_empty() {
        return;
    }

    loop {
        let city = get_city_name();
        let country_code = get_country_code();

        let weather_info = get_weather_info(&city, &country_code, &api_key);

        match weather_info {
            Ok(response) => {
                print_weather_info(response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        let choice = is_repeat();

        if !choice {
            break;
        }
    }
}

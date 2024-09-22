use std::io;
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String
}
#[derive(Deserialize, Debug)]
struct Weather {
    description: String
}
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64
}
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64
}


fn get_weather_api(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}


fn display_weather_info(response: &WeatherResponse) {
    // Extracting weather information from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather information into a string
    let weather_text = format!(
"Weather in {}: {} {}
> Temperature: {:.1}°C, 
> Humidity: {:.1}%, 
> Pressure: {:.1} hPa, 
> Wind Speed: {:.1} m/s",
    response.name,
    description,
    get_temperature_emoji(temperature),
    temperature,
    humidity,
    pressure,
    wind_speed,
);

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);
}


fn get_temperature_emoji(temp: f64)-> &'static str{
    if temp < 0.0 {
        "❄️"
    }else if temp>=0.0 && temp<10.0 {
        "☁️"
    } else if temp >= 10.0 && temp < 20.0 {
        "⛅"
    } else if temp >= 20.0 && temp < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main(){
    println!("{}", "Welcome to Weather Station!".bright_yellow());
    

   loop {
    println!("{}", "Please enter the name of the city:".bright_green());
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("faied to read input");
    let city = city.trim();

    println!("{}", "Please enter the country code (e.g., US for United States):".bright_green());
    let mut country_code = String::new();
    io::stdin().read_line(&mut country_code).expect("failed to read input");
    let country_code = country_code.trim();

    let api_key = "f04af4cd0d5b223b661d6104430f2e99"; 

    match get_weather_api(city, country_code, api_key) {
        Ok(respose)=>{
            display_weather_info(&respose);
        }
        Err(err) => {
            eprint!("Error: {}", err);
        }
    }

    // print!("{}", "do you want to continue? (Yes/No)");
    println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    let input = input.trim().to_lowercase();


    if input!="yes" {
        println!("Thank you for using our software!");
        break;
    }
   }

}
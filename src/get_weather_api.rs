use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
}
#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}
#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub humidity: f64,
    pub pressure: f64,
}
#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
}

pub fn get_weather_api(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

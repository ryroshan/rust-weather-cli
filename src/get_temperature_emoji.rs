
pub fn get_temperature_emoji(temp: f64)-> &'static str{
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

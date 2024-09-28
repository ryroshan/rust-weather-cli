use std::io;
use colored::Colorize;
mod get_weather_api;
mod display_weather_info;
mod get_temperature_emoji;
use get_weather_api::get_weather_api;
use display_weather_info::display_weather_info;
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
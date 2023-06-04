// Eg. http://api.weatherapi.com/v1/current.json?key=a47a552967c84f1ba3700150232305&q=Kitchener&aqi=no
use std::env;
use reqwest::Url;
use reqwest::Error;
use tokio;

fn construct_request(query: &str, param: &str, aqi: &str) -> Result<Url, Box<dyn std::error::Error>> {
    const BASE_URL: &str = "http://api.weatherapi.com/v1";
    let api_key = env::var("API_KEY").expect("API_KEY variable not found");

    let mut url_string = format!("{}/{}?key={}&q={}&{}", BASE_URL, param, api_key, query, aqi);
    // create url object
    let url = Url::parse(&url_string)?;

    Ok(url)
}

async fn get_request(url: &str) -> Result<(), Error>{
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    println!("body = {:?}", body);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let q = match args.get(1) {
        Some(q) => q,
        None => {
            println!("Input a valid city name");
            return;
        }
    };
    let param = match args.get(2) {
        Some(param) => param.as_str(),
        None => {
            println!("Using defaults");
            "current.json"
        }
    };
    let aqi = match args.get(3) {
        Some(aqi) => aqi.as_str(),
        None => {
            "aqi=no"
        }
    };
    let request_url = match construct_request(&q, param, aqi) {
        Ok(url) => url,
        Err(e) => {
            println!("Error constructing request URL: {:?}", e);
            return;
        }
    };

    get_request(&request_url.to_string());
}

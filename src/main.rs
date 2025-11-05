use std::env;
use dotenv::dotenv;
use reqwest::blocking;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    message: String,
    result: String,  
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("ETHERSCAN_API_KEY")
        .expect("Missing ETHERSCAN_API_KEY in .env");
        println!("Please enter the address to which you wish to avail the balance!");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).expect("Value not entered right");  

    let url = format!(
        "https://api.etherscan.io/v2/api?chainid=1&module=account&action=balance&address={}&apikey={}",
        address, api_key
    );

    let response = blocking::get(&url)?
        .json::<ApiResponse>()?;

    if response.status == "1" {
        let wei: f64 = response.result.parse().expect("Failed parsing");
        let eth = wei / 1e18;
        println!("Balance for {}: {} ETH", address, eth);
    } else {
        println!("API returned error: message={} result={}", response.message, response.result);
    }

    Ok(())
}

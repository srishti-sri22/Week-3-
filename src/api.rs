use crate::models::{BalanceResponse, TransactionsResponse};
use reqwest::blocking;
use std::env;

pub fn get_balance(address: &str) -> Result<BalanceResponse, reqwest::Error> {
    let api_key = env::var("ETHERSCAN_API_KEY").expect("The API key cannot be accessed.");
    let url = format!(
        "https://api.etherscan.io/v2/api?chainid=1&module=account&action=balance&address={}&apikey={}",
        address, api_key
    );

    // let response_text = blocking::get(&url)?.text()?;
    // println!("API response: {}", response_text);

    let response =  blocking::get(&url).expect("Could not fetch the result")
        .json::<BalanceResponse>().expect("The matching style does not match");
    Ok(response)
}

pub fn get_transactions(address: &str, page:u32, offset:u32) -> Result<TransactionsResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("ETHERSCAN_API_KEY")?;
    let url = format!(
        "https://api.etherscan.io/v2/api?chainid=1&module=account&action=txlist&address={}&page={}&offset={}&apikey={}",
        address, page, offset, api_key
    );

    // let response_text = blocking::get(&url)?.text()?;
    // println!("API response: {}", response_text);

    let response =  blocking::get(&url).expect("Could not fetch the result")
        .json::<TransactionsResponse>().expect("The matching style does not match");
    Ok(response)
}

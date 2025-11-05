use reqwest::blocking;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "UPX7UIWMWKFN9U49ZYXQZYMC1BBYA8XAZJ";
    let tx_hash = "0x8f7fe8dc882776af19e3a759bd00aa370d3925f852ab534a13a94bd0f6218c54";

    let url = format!(
        "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getTransactionByHash&txhash={}&apikey={}",
        tx_hash, api_key
    );

    let response = blocking::get(&url)?;
    let json: Value = response.json()?; // requires JSON feature in Cargo.toml

    println!("Transaction details: {:#?}", json);

    Ok(())
}

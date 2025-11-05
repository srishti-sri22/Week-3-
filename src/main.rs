mod api;
mod models;

use std::io;
use dotenv::dotenv;

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let mut address = String::new();
    println!("Enter the Ethereum address:");
    io::stdin().read_line(&mut address).expect("Please enter an appropriate address.");
    let address = address.trim();

    let mut page_input = String::new();
    println!("Enter desired no. of pages:");
    io::stdin().read_line(&mut page_input).expect("Please enter an appropriate address.");
    let page: u32 = page_input.trim().parse().expect("Paarsing failed to u32");

    let mut offset_input = String::new();
    println!("Enter the no. of transactions per page you want:");
    io::stdin().read_line(&mut offset_input).expect("Please enter an appropriate address.");
    let offset: u32 = offset_input.trim().parse().expect("Parsing failed to u32");


    let balance_response = api::get_balance(address).expect("Cannot fetch the balance!");
    if balance_response.status == "1" {
        let wei: u128 = balance_response.result.parse().expect("Parsing failed!");
        let eth = wei as f64 / 1e18;
        println!("Balance for {}: {} ETH", address, eth);
    } else {
        println!("Failed to fetch balance: {}", balance_response.message);
    }

    let mut current_page = 1;
    while current_page <= page {
        let tx_response = api::get_transactions(address, current_page, offset).expect("Cannot fetch the transactions!");

        if tx_response.result.is_empty() {
            println!("No more transactions found.");
            break;
        }

        println!("Transactions - Page {}:", current_page);
        for tx in &tx_response.result {
            println!("Hash: {}, From: {}, To: {}, Value: {}", tx.hash, tx.from, tx.to, tx.value
            );
        }

        current_page += 1;
    }
    Ok(())
}

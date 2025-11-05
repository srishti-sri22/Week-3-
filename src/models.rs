use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BalanceResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<Transaction>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct Transaction {
    pub _block_number: String,
    pub _time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub _gas: String,
    pub _gas_price: String,
}

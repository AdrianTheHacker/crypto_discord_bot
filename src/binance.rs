use reqwest;
use serde::{Serialize, Deserialize};

 
#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceApiResponce { // Information given from binance.
    symbol: String,
    price: String,
}


pub fn get_crypto_price(symbol: &str) -> BinanceApiResponce {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let res = reqwest::blocking::get(&url).expect("Can't access api").json().unwrap();
    
    let binance_api_responce: BinanceApiResponce = res;
    return binance_api_responce;
}

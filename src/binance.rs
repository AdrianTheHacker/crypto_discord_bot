use serde::{Serialize, Deserialize};

 
#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceApiResponce { // Information given from binance.
    pub symbol: String,
    pub price: String,
}


pub async fn get_crypto_price(symbol: &str) -> BinanceApiResponce {
    // Returns the price of a given crypto currency as USDT.
    // The price of the crypto is grabbed from api.binance.com

    let url: String = format!("https://api.binance.com/api/v3/ticker/price?symbol={}USDT", symbol);
    
    let binance_api_responce: BinanceApiResponce = tokio::task::spawn_blocking(move || {
        // do some compute-heavy work or call synchronous code
        let res: BinanceApiResponce = reqwest::blocking::get(&url).expect("Can't access api").json().unwrap();
    
        let binance_api_responce: BinanceApiResponce = res;
        binance_api_responce
    }).await.unwrap();
                                   
    binance_api_responce
}

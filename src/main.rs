mod binance;
mod discord_bot;


#[tokio::main]
async fn main() {
    discord_bot::start_bot().await;
}

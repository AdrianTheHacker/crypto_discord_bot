use env_file_reader::read_file;

use std::collections::HashMap;

mod binance;
mod discord_bot;

#[tokio::main]
async fn main() {
    let env_variables: HashMap<String, String> = read_file(r".env").unwrap();

    discord_bot::start_bot(&env_variables["DISCORD_BOT_TOKEN"]).await;
}

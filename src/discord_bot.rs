use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[path = "binance.rs"]
mod binance;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!get_bitcoin" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.

            println!("Running Command: !get_bitcoin");
            println!("Fetching bitcoin data...");
            
            let binance_api_responce: binance::BinanceApiResponce = binance::get_crypto_price(&"BTC").await;
            let crypto_data: String = format!("{}: {}", binance_api_responce.symbol, binance_api_responce.price);

            if let Err(why) = msg.channel_id.say(&ctx.http, &crypto_data).await {
                println!("Error sending message: {:?}", why);
            }

            println!("{:?}", &crypto_data);
        }

        if msg.content == "very pog" {
            println!("Running Command: very pog");

            if let Err(why) = msg.channel_id.say(&ctx.http, "indeed").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


pub async fn start_bot(bot_key: &str) {
    // Configure the client with your Discord bot token in the environment.
    let token: &str = bot_key;
    // Set gateway intents, which decides what events the bot will be notified about
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
                                | GatewayIntents::DIRECT_MESSAGES
                                | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client: Client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

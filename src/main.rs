mod handler;
mod commands;
mod cmds;
mod components;

use serenity::{all::GatewayIntents, client::Client};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let token: String = std::env::var("TOKEN").expect("[Client] -> Missing token");
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("[Client] ->");

    if let Err(why) = client.start().await {
        println!("[Client] -> {why:?}");
    }
}

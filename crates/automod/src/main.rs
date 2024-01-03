mod args;
mod bot;
mod config;

use crate::bot::Bot;
use config::Config;
use lemmy_client::Client;
use std::process::exit;

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args = args::parse();

    // Parse configuration
    let filepath = args.get_one::<String>(args::CONFIG).unwrap();
    let config = match Config::load(filepath) {
        None => {
            println!("missing or invalid configuration!");
            exit(1);
        }
        Some(result) => result,
    };

    // Create API client with configured credentials
    let bot_user = config.lemmy;
    let client = match Client::new(bot_user.host, bot_user.username, bot_user.password).await {
        Ok(client) => client,
        Err(err) => {
            println!("failed to initialize client: {}", err);
            exit(1);
        }
    };

    // Create and run bot
    let mut bot = Bot::new(config.plugins);
    bot.run(client).await;
}

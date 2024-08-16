use akashi_cmds::register_all_commands;
use akashi_shared::framework::options::{initialize_poise_framework, initialize_poise_options};
use poise::serenity_prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env variable not found");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let poise_options = initialize_poise_options(register_all_commands());
    let poise_framework = initialize_poise_framework(poise_options);

    let mut cache_settings = Settings::default();

    cache_settings.cache_users = false;
    cache_settings.cache_channels = false;

    let client = ClientBuilder::new(discord_token, intents)
        .framework(poise_framework)
        .cache_settings(cache_settings)
        .await;

    client.unwrap().start().await.unwrap()
}

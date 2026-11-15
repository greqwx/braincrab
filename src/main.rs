// Copyright (c) 2026 Greg, l1acy

mod audit;
mod commands;
mod config;
mod consts;
mod handler;

use std::sync::Arc;

use serenity::cache::Settings;
use serenity::prelude::{Client, GatewayIntents};

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut cache_settings = Settings::default();
    cache_settings.max_messages = 200;

    let mut client = Client::builder(config::token(), intents)
        .cache_settings(cache_settings)
        .event_handler(Arc::new(handler::Handler))
        .await
        .expect("не удалось создать клиент");

    if let Err(why) = client.start().await {
        eprintln!("ошибка клиента: {why}");
    }
}

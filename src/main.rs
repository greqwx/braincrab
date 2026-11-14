// Copyright (c) 2026 Greg, l1acy

mod commands;
mod config;
mod consts;
mod handler;

use std::sync::Arc;

use serenity::prelude::{Client, GatewayIntents};

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::empty();

    let mut client = Client::builder(config::token(), intents)
        .event_handler(Arc::new(handler::Handler))
        .await
        .expect("не удалось создать клиент");

    if let Err(why) = client.start().await {
        eprintln!("ошибка клиента: {why}");
    }
}

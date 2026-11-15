// Copyright (c) 2026 Greg, l1acy

use serenity::all::{FullEvent, Interaction};
use serenity::async_trait;
use serenity::prelude::{Context, EventHandler};

use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
        crate::audit::handle(ctx, event);

        match event {
            FullEvent::Ready { data_about_bot, .. } => {
                println!("вход выполнен под именем {}", data_about_bot.user.name);

                if let Err(why) = commands::register_all(ctx).await {
                    eprintln!("ошибка регистрации команд: {why}");
                }
            },
            FullEvent::InteractionCreate { interaction, .. } => {
                if let Interaction::Command(command) = interaction {
                    commands::dispatch(ctx, command).await;
                }
            },
            _ => {},
        }
    }
}

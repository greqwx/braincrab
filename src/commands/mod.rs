// Copyright (c) 2026 Greg, l1acy

mod avatar_command;
mod bot_command;
mod help_command;
mod ping_command;
mod registry;

use serenity::all::{Command, CommandInteraction};
use serenity::prelude::Context;

pub async fn register_all(ctx: &Context) -> Result<(), serenity::Error> {
    for register in registry::ALL {
        Command::create_global_command(&ctx.http, register()).await?;
    }
    Ok(())
}

pub async fn dispatch(ctx: &Context, interaction: &CommandInteraction) {
    let result = match interaction.data.name.as_str() {
        "avatar" => avatar_command::run(ctx, interaction).await,
        "bot" => bot_command::run(ctx, interaction).await,
        "help" => help_command::run(ctx, interaction).await,
        "ping" => ping_command::run(ctx, interaction).await,
        _ => return,
    };

    if let Err(why) = result {
        eprintln!("ошибка обработки команды {}: {why}", interaction.data.name);
    }
}

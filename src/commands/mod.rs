// Copyright (c) 2026 Greg, l1acy

mod avatar_command;
mod bot_command;
mod ping_command;

use serenity::all::{Command, CommandInteraction};
use serenity::prelude::Context;

pub async fn register_all(ctx: &Context) -> Result<(), serenity::Error> {
    Command::create_global_command(&ctx.http, bot_command::register()).await?;
    Command::create_global_command(&ctx.http, avatar_command::register()).await?;
    Command::create_global_command(&ctx.http, ping_command::register()).await?;
    Ok(())
}

pub async fn dispatch(ctx: &Context, interaction: &CommandInteraction) {
    let result = match interaction.data.name.as_str() {
        "bot" => bot_command::run(ctx, interaction).await,
        "avatar" => avatar_command::run(ctx, interaction).await,
        "ping" => ping_command::run(ctx, interaction).await,
        _ => return,
    };

    if let Err(why) = result {
        eprintln!("ошибка обработки команды {}: {why}", interaction.data.name);
    }
}

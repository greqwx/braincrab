// Copyright (c) 2026 Greg, l1acy

mod bot_command;

use serenity::all::{Command, CommandInteraction};
use serenity::prelude::Context;

pub async fn register_all(ctx: &Context) -> Result<(), serenity::Error> {
    Command::create_global_command(&ctx.http, bot_command::register()).await?;
    Ok(())
}

pub async fn dispatch(ctx: &Context, interaction: &CommandInteraction) {
    let result = match interaction.data.name.as_str() {
        "bot" => bot_command::run(ctx, interaction).await,
        _ => return,
    };

    if let Err(why) = result {
        eprintln!("ошибка обработки команды {}: {why}", interaction.data.name);
    }
}

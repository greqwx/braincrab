// Copyright (c) 2026 Greg, l1acy

mod avatar_command;
mod bot_command;
mod help_command;
mod ping_command;
mod purge_command;
mod registry;
mod sysinfo_command;
mod userinfo_command;

use serenity::all::{Command, CommandInteraction};
use serenity::prelude::Context;

pub async fn register_all(ctx: &Context) -> Result<(), serenity::Error> {
    for entry in registry::ALL {
        let registry::Entry::Command(entry) = entry else {
            continue;
        };

        Command::create_global_command(&ctx.http, (entry.register)()).await?;
    }
    Ok(())
}

pub async fn dispatch(ctx: &Context, interaction: &CommandInteraction) {
    let result = match interaction.data.name.as_str() {
        avatar_command::NAME => avatar_command::run(ctx, interaction).await,
        bot_command::NAME => bot_command::run(ctx, interaction).await,
        help_command::NAME => help_command::run(ctx, interaction).await,
        ping_command::NAME => ping_command::run(ctx, interaction).await,
        purge_command::NAME => purge_command::run(ctx, interaction).await,
        sysinfo_command::NAME => sysinfo_command::run(ctx, interaction).await,
        userinfo_command::NAME => userinfo_command::run(ctx, interaction).await,
        _ => return,
    };

    if let Err(why) = result {
        eprintln!("ошибка обработки команды {}: {why}", interaction.data.name);
    }
}

// Copyright (c) 2026 Greg, l1acy

use std::time::{SystemTime, UNIX_EPOCH};

use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateCommand,
    CreateCommandOption, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateTextDisplay, GetMessages,
    MessageFlags, MessageId, UserId,
};
use serenity::prelude::Context;

use crate::consts::MODERATOR_ROLES;

/// дискорд не удалит больше 100 сообщений за раз
const BULK_LIMIT: usize = 100;

const MAX_SCAN: usize = 500;
const TWO_WEEKS: i64 = 14 * 24 * 60 * 60;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("purge")
        .description("Удалить сообщения в канале")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "amount",
                "Сколько сообщений удалить (1-100)",
            )
            .min_int_value(1)
            .max_int_value(BULK_LIMIT as i64)
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "Удалять только сообщения этого участника",
            )
            .required(false),
        )
}

#[derive(Default)]
struct PurgeOptions {
    amount: Option<usize>,
    user: Option<UserId>,
}

fn parse_options(interaction: &CommandInteraction) -> PurgeOptions {
    let mut options = PurgeOptions::default();

    for option in &interaction.data.options {
        match (&option.value, option.name.as_str()) {
            (CommandDataOptionValue::Integer(amount), "amount") => {
                options.amount = Some((*amount).max(1) as usize);
            }
            (CommandDataOptionValue::User(user_id), "user") => options.user = Some(*user_id),
            _ => {}
        }
    }

    options
}

async fn collect_ids(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &PurgeOptions,
) -> Result<(Vec<MessageId>, usize), serenity::Error> {
    let wanted = options.amount.unwrap_or(BULK_LIMIT);
    let cutoff = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .unwrap_or(0)
        - TWO_WEEKS;

    let mut ids = Vec::with_capacity(wanted);
    let mut skipped = 0;
    let mut before = None;
    let mut scanned = 0;

    while ids.len() < wanted && scanned < MAX_SCAN {
        let mut builder = GetMessages::new().limit(BULK_LIMIT as u8);
        if let Some(message_id) = before {
            builder = builder.before(message_id);
        }

        let messages = interaction.channel_id.messages(&ctx.http, builder).await?;
        let page_len = messages.len();
        if page_len == 0 {
            break;
        }

        before = messages.last().map(|message| message.id);
        scanned += page_len;

        for message in messages {
            if options
                .user
                .is_some_and(|user_id| message.author.id != user_id)
            {
                continue;
            }

            if message.timestamp.unix_timestamp() < cutoff {
                skipped += 1;
                continue;
            }

            ids.push(message.id);
            if ids.len() == wanted {
                break;
            }
        }

        if page_len < BULK_LIMIT {
            break;
        }
    }

    Ok((ids, skipped))
}

async fn respond(
    ctx: &Context,
    interaction: &CommandInteraction,
    text: &str,
) -> Result<(), serenity::Error> {
    let container = CreateContainer::new(vec![
        CreateContainerComponent::TextDisplay(CreateTextDisplay::new("`/commands/purge`")),
        CreateContainerComponent::TextDisplay(CreateTextDisplay::new(text)),
    ]);
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2 | MessageFlags::EPHEMERAL);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    if interaction.guild_id.is_none() {
        return respond(ctx, interaction, "команда работает только на сервере").await;
    }

    let roles = interaction
        .member
        .as_ref()
        .map(|member| member.roles.as_slice())
        .unwrap_or_default();

    if !roles.iter().any(|role| MODERATOR_ROLES.contains(role)) {
        return respond(ctx, interaction, "команда доступна только модераторам").await;
    }

    if !interaction.app_permissions.manage_messages() {
        return respond(
            ctx,
            interaction,
            "мне не хватает права `Manage Messages` в этом канале",
        )
        .await;
    }

    let options = parse_options(interaction);
    if options.amount.is_none() && options.user.is_none() {
        return respond(ctx, interaction, "укажи `amount` или `user`, хотя бы одно").await;
    }

    let (ids, skipped) = collect_ids(ctx, interaction, &options).await?;
    if ids.is_empty() {
        return respond(ctx, interaction, "нечего удалять").await;
    }

    if let Err(why) = interaction
        .channel_id
        .delete_messages(&ctx.http, &ids, None)
        .await
    {
        return respond(
            ctx,
            interaction,
            &format!("не удалось удалить сообщения: {why}"),
        )
        .await;
    }

    let mut text = format!("удалено сообщений: `{}`", ids.len());
    if skipped > 0 {
        text.push_str(&format!("\nпропущено старше 14 дней: `{skipped}`"));
    }

    respond(ctx, interaction, &text).await
}

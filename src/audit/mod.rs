// Copyright (c) 2026 Greg, l1acy

mod member;
mod message;
mod render;
mod voice;

use serenity::all::{FullEvent, GuildId};
use serenity::prelude::Context;

use crate::consts::{AUDIT_GUILD_ID, AUDIT_LOG_CHANNEL};

/// собирает запись по событию и отправляет её в лог-канал
pub fn handle(ctx: &Context, event: &FullEvent) {
    let Some((guild_id, body)) = build(ctx, event) else {
        return;
    };

    if guild_id != AUDIT_GUILD_ID {
        return;
    }

    let http = ctx.http.clone();
    tokio::spawn(async move {
        if let Err(why) = render::send(&http, AUDIT_LOG_CHANNEL, &body).await {
            eprintln!("не удалось отправить запись аудита: {why}");
        }
    });
}

fn build(ctx: &Context, event: &FullEvent) -> Option<(GuildId, String)> {
    match event {
        FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => message::deleted(ctx, (*guild_id)?, *channel_id, *deleted_message_id),
        FullEvent::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids,
            guild_id,
        } => message::bulk_deleted(ctx, (*guild_id)?, *channel_id, multiple_deleted_messages_ids),
        FullEvent::MessageUpdate {
            old_if_available,
            event,
        } => message::edited(old_if_available.as_ref(), &event.message),
        FullEvent::GuildMemberAddition { new_member } => member::joined(new_member),
        FullEvent::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available,
        } => member::left(ctx, *guild_id, user, member_data_if_available.as_ref()),
        FullEvent::GuildMemberUpdate {
            old_if_available,
            event,
            ..
        } => member::updated(ctx, old_if_available.as_ref(), event),
        FullEvent::VoiceStateUpdate { old, new } => voice::changed(old.as_ref(), new),
        _ => None,
    }
}

// Copyright (c) 2026 Greg, l1acy

use std::collections::BTreeMap;

use serenity::all::{GenericChannelId, GuildId, Message, MessageId, UserId};
use serenity::prelude::Context;

use super::render::{attachments, body, channel, timestamp, user};

const MAX_AUTHORS: usize = 8;

pub fn deleted(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: GenericChannelId,
    message_id: MessageId,
) -> Option<(GuildId, String)> {
    let mut lines = vec![
        "**сообщение удалено**".to_owned(),
        format!("канал {}", channel(channel_id)),
    ];

    match ctx.cache.message(channel_id, message_id) {
        Some(message) => {
            lines.push(format!(
                "автор {} (`{}`)",
                user(message.author.id),
                message.author.name
            ));
            lines.push(format!("отправлено {}", timestamp(message.timestamp)));
            lines.push(body(&message.content));

            if let Some(files) = attachments(&message) {
                lines.push(files);
            }
        }
        None => lines.push("содержимое недоступно: сообщения не было в кэше".to_owned()),
    }

    Some((guild_id, lines.join("\n")))
}

pub fn bulk_deleted(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: GenericChannelId,
    message_ids: &[MessageId],
) -> Option<(GuildId, String)> {
    if message_ids.is_empty() {
        return None;
    }

    let mut authors: BTreeMap<UserId, (String, usize)> = BTreeMap::new();
    for message_id in message_ids {
        let Some(message) = ctx.cache.message(channel_id, *message_id) else {
            continue;
        };

        let entry = authors
            .entry(message.author.id)
            .or_insert_with(|| (message.author.name.to_string(), 0));
        entry.1 += 1;
    }

    let mut lines = vec![
        "**массовое удаление**".to_owned(),
        format!("канал {}", channel(channel_id)),
        format!("сообщений: `{}`", message_ids.len()),
    ];

    if !authors.is_empty() {
        let mut list = authors
            .iter()
            .take(MAX_AUTHORS)
            .map(|(id, (name, count))| format!("{} (`{name}`) — `{count}`", user(*id)))
            .collect::<Vec<_>>()
            .join(", ");

        if authors.len() > MAX_AUTHORS {
            let rest = authors.len() - MAX_AUTHORS;
            list.push_str(&format!(" и ещё `{rest}`"));
        }

        lines.push(format!("авторы по кэшу: {list}"));
    }

    Some((guild_id, lines.join("\n")))
}

pub fn edited(old: Option<&Message>, new: &Message) -> Option<(GuildId, String)> {
    let old = old?;
    let guild_id = new.guild_id?;

    let content_changed = old.content != new.content;
    let old_files = files(old);
    let new_files = files(new);

    if !content_changed && old_files == new_files {
        return None;
    }

    let mut lines = vec![
        "**сообщение изменено**".to_owned(),
        format!(
            "канал {} · автор {} (`{}`)",
            channel(new.channel_id),
            user(new.author.id),
            new.author.name
        ),
        format!("ссылка <{}>", new.link()),
    ];

    if content_changed {
        lines.push("было:".to_owned());
        lines.push(body(&old.content));
        lines.push("стало:".to_owned());
        lines.push(body(&new.content));
    }

    if old_files != new_files {
        lines.push(format!("файлы: {old_files} → {new_files}"));
    }

    Some((guild_id, lines.join("\n")))
}

fn files(message: &Message) -> String {
    if message.attachments.is_empty() {
        return "нет".to_owned();
    }

    message
        .attachments
        .iter()
        .map(|attachment| format!("`{}`", attachment.filename))
        .collect::<Vec<_>>()
        .join(", ")
}

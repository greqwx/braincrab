// Copyright (c) 2026 Greg, l1acy

use std::fmt::Display;

use serenity::all::{
    ChannelId, CreateAllowedMentions, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateMessage, CreateSeparator, CreateTextDisplay, Http, Message, MessageFlags, Timestamp,
};

/// больше этого текст удалённого или старого сообщения не показываем
const MAX_CONTENT: usize = 900;
const MAX_ATTACHMENTS: usize = 5;

pub async fn send(http: &Http, channel_id: ChannelId, body: &str) -> Result<(), serenity::Error> {
    let container = CreateContainer::new(vec![
        CreateContainerComponent::TextDisplay(CreateTextDisplay::new("`/audit`")),
        CreateContainerComponent::Separator(CreateSeparator::new()),
        CreateContainerComponent::TextDisplay(CreateTextDisplay::new(body.to_owned())),
    ]);
    let message = CreateMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .allowed_mentions(CreateAllowedMentions::new())
        .flags(MessageFlags::IS_COMPONENTS_V2);

    channel_id.widen().send_message(http, message).await?;
    Ok(())
}

pub fn channel(id: impl Display) -> String {
    format!("<#{id}>")
}

pub fn user(id: impl Display) -> String {
    format!("<@{id}>")
}

pub fn role(id: impl Display) -> String {
    format!("<@&{id}>")
}

pub fn timestamp(time: Timestamp) -> String {
    format!("<t:{}:f>", time.unix_timestamp())
}

/// оборачивает текст в блок кода, чтобы разметка из сообщения не ломала лог
pub fn quote(text: &str) -> String {
    let text = shorten(text, MAX_CONTENT);

    if text.contains("```") {
        format!("````\n{text}\n````")
    } else {
        format!("```\n{text}\n```")
    }
}

pub fn body(text: &str) -> String {
    if text.trim().is_empty() {
        "текста нет".to_owned()
    } else {
        quote(text)
    }
}

pub fn attachments(message: &Message) -> Option<String> {
    let count = message.attachments.len() as usize;
    if count == 0 {
        return None;
    }

    let names = message
        .attachments
        .iter()
        .take(MAX_ATTACHMENTS)
        .map(|attachment| format!("`{}`", attachment.filename))
        .collect::<Vec<_>>()
        .join(", ");

    let mut text = format!("файлы: {names}");
    if count > MAX_ATTACHMENTS {
        let rest = count - MAX_ATTACHMENTS;
        text.push_str(&format!(" и ещё `{rest}`"));
    }

    Some(text)
}

fn shorten(text: &str, limit: usize) -> String {
    let mut shortened: String = text.chars().take(limit).collect();
    if shortened.len() < text.len() {
        shortened.push('…');
    }
    shortened
}

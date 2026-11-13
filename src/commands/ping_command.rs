// Copyright (c) 2026 Greg, l1acy

use std::time::Instant;

use serenity::all::{
    CommandInteraction, CreateCommand, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateTextDisplay, Http,
    MessageFlags,
};
use serenity::prelude::Context;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("ping").description("Проверить задержку бота")
}

async fn measure_latency(http: &Http) -> Option<u128> {
    let started = Instant::now();
    http.get_current_user().await.ok()?;
    Some(started.elapsed().as_millis())
}

fn build_container(latency: Option<u128>) -> CreateContainer<'static> {
    let header = CreateTextDisplay::new("`/commands/ping`");
    let body = match latency {
        Some(ms) => CreateTextDisplay::new(format!("понг! задержка: `{ms} ms`")),
        None => CreateTextDisplay::new("понг! не удалось измерить задержку"),
    };

    CreateContainer::new(vec![
        CreateContainerComponent::TextDisplay(header),
        CreateContainerComponent::TextDisplay(body),
    ])
    .accent_color(0xf64c02)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let container = build_container(measure_latency(&ctx.http).await);
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

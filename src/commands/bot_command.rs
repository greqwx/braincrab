// Copyright (c) 2026 Greg, l1acy

use serenity::all::{
    CommandInteraction, CreateCommand, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateSeparator,
    CreateTextDisplay, MessageFlags,
};
use serenity::prelude::Context;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("bot").description("Показать информацию о боте")
}

fn build_container() -> CreateContainer<'static> {
    let header = CreateTextDisplay::new("`/commands/bot`");

    let body = CreateTextDisplay::new(
        "привет! я open-source бот, написанный на Rust.\nсделано с любовью от людей для людей <3",
    );
    let code_body = CreateTextDisplay::new("репозиторий: [greqwx/braincrab](https://github.com/greqwx/braincrab)\nлицензия: MIT License");

    let versions = CreateTextDisplay::new("версия бота: `1.0-dev`\nверсия rust: `1.99.1`");
    let footer = CreateTextDisplay::new("-# Copyright (c) 2026 Greg, l1acy");

    CreateContainer::new(vec![
        CreateContainerComponent::TextDisplay(header),
        CreateContainerComponent::TextDisplay(body),
        CreateContainerComponent::Separator(CreateSeparator::new()),
        CreateContainerComponent::TextDisplay(code_body),
        CreateContainerComponent::Separator(CreateSeparator::new()),
        CreateContainerComponent::TextDisplay(versions),
        CreateContainerComponent::TextDisplay(footer),
    ])
    .accent_color(0xf64c02)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let container = build_container();
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

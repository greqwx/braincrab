// Copyright (c) 2026 Greg, l1acy

use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateCommand,
    CreateCommandOption, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMediaGallery,
    CreateMediaGalleryItem, CreateTextDisplay, CreateUnfurledMediaItem, MessageFlags,
};
use serenity::prelude::Context;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("avatar")
        .description("Показать аватар пользователя")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "Чей аватар показать")
                .required(false),
        )
}

fn build_container(display_name: &str, avatar_url: &str) -> CreateContainer<'static> {
    let header = CreateTextDisplay::new(format!("`/commands/avatar`"));
    let body = CreateTextDisplay::new(format!("показан аватар **{display_name}:**"));
    let media = CreateUnfurledMediaItem::new(avatar_url.to_string());
    let gallery = CreateMediaGallery::new(vec![CreateMediaGalleryItem::new(media)]);

    CreateContainer::new(vec![
        CreateContainerComponent::TextDisplay(header),
        CreateContainerComponent::TextDisplay(body),
        CreateContainerComponent::MediaGallery(gallery),
    ])
    .accent_color(0xf64c02)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let target_user = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "user")
        .and_then(|option| match &option.value {
            CommandDataOptionValue::User(user_id) => interaction.data.resolved.users.get(user_id),
            _ => None,
        });

    let (display_name, avatar_url) = match target_user {
        Some(user) => (user.name.clone(), user.face()),
        None => (interaction.user.name.clone(), interaction.user.face()),
    };

    let container = build_container(&display_name, &avatar_url);
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

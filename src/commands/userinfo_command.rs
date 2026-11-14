// Copyright (c) 2026 Greg, l1acy

use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CreateCommand,
    CreateCommandOption, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateSection,
    CreateSectionAccessory, CreateSectionComponent, CreateSeparator, CreateTextDisplay,
    CreateThumbnail, CreateUnfurledMediaItem, Mentionable, MessageFlags, RoleId, Timestamp, User,
};
use serenity::prelude::Context;

/// Общие для `Member` и `PartialMember` поля, которые нужны этой команде.
struct GuildMemberInfo<'a> {
    nick: Option<&'a str>,
    joined_at: Option<Timestamp>,
    roles: &'a [RoleId],
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("userinfo")
        .description("Показать информацию о пользователе")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "О ком показать информацию")
                .required(false),
        )
}

fn account_kind(user: &User) -> &'static str {
    if user.bot() {
        "бот"
    } else if user.system() {
        "системный аккаунт discord"
    } else {
        "пользователь"
    }
}

struct UserinfoSections {
    /// Каждый элемент - отдельный блок полей, между ними в контейнере встанет Separator.
    fields: Vec<String>,
}

fn build_sections(user: &User, member: Option<GuildMemberInfo<'_>>) -> UserinfoSections {
    let tag = user.tag();
    let created_at = user.id.created_at().unix_timestamp();

    let identity = [
        format!("тег: `{tag}`"),
        format!("id: `{}`", user.id),
        format!("тип: `{}`", account_kind(user)),
    ]
    .join("\n");

    let names = {
        let mut lines = Vec::new();

        if let Some(global_name) = user.global_name.as_ref() {
            lines.push(format!("отображаемое имя: `{global_name}`"));
        }

        if let Some(nick) = member.as_ref().and_then(|member| member.nick) {
            lines.push(format!("ник на сервере: `{nick}`"));
        }

        lines.join("\n")
    };

    let dates = {
        let mut lines = vec![format!("аккаунт создан: <t:{created_at}:R>")];

        if let Some(joined_at) = member.as_ref().and_then(|member| member.joined_at) {
            lines.push(format!(
                "на сервере с: <t:{}:R>",
                joined_at.unix_timestamp()
            ));
        }

        lines.join("\n")
    };

    let mut fields = vec![identity];
    if !names.is_empty() {
        fields.push(names);
    }
    fields.push(dates);

    if let Some(roles) = member.and_then(|member| {
        if member.roles.is_empty() {
            return None;
        }

        let mentions = member
            .roles
            .iter()
            .map(|role_id| role_id.mention().to_string())
            .collect::<Vec<_>>()
            .join(" ");

        Some(mentions)
    }) {
        fields.push(roles);
    }

    UserinfoSections { fields }
}

fn build_container(
    display_name: &str,
    avatar_url: &str,
    sections: UserinfoSections,
) -> CreateContainer<'static> {
    let header = CreateTextDisplay::new("`/commands/userinfo`");

    let intro_text = format!("информация о **{display_name}**\n\nниже предоставлена небольшая\nинформация о этом пользователе");

    let intro = CreateSectionComponent::TextDisplay(CreateTextDisplay::new(intro_text));
    let thumbnail = CreateSectionAccessory::Thumbnail(CreateThumbnail::new(
        CreateUnfurledMediaItem::new(avatar_url.to_string()),
    ));
    let intro_section = CreateSection::new(vec![intro], thumbnail);

    let mut components = vec![
        CreateContainerComponent::TextDisplay(header),
        CreateContainerComponent::Section(intro_section),
    ];

    for field_block in sections.fields {
        components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
        components.push(CreateContainerComponent::TextDisplay(
            CreateTextDisplay::new(field_block),
        ));
    }

    CreateContainer::new(components)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let target_user_id = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "user")
        .and_then(|option| match &option.value {
            CommandDataOptionValue::User(user_id) => Some(*user_id),
            _ => None,
        });

    let (user, member): (&User, Option<GuildMemberInfo<'_>>) = match target_user_id {
        Some(user_id) => {
            let user = interaction
                .data
                .resolved
                .users
                .get(&user_id)
                .unwrap_or(&interaction.user);
            let member = interaction
                .data
                .resolved
                .members
                .get(&user_id)
                .map(|member| GuildMemberInfo {
                    nick: member.nick.as_deref(),
                    joined_at: member.joined_at,
                    roles: &member.roles,
                });
            (user, member)
        }
        None => {
            let member = interaction.member.as_deref().map(|member| GuildMemberInfo {
                nick: member.nick.as_deref(),
                joined_at: member.joined_at,
                roles: &member.roles,
            });
            (&interaction.user, member)
        }
    };

    let display_name = user.display_name().to_string();
    let avatar_url = user.face();
    let sections = build_sections(user, member);

    let container = build_container(&display_name, &avatar_url, sections);
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

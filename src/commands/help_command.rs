// Copyright (c) 2026 Greg, l1acy

use serenity::all::{
    Command, CommandInteraction, CreateCommand, CreateComponent, CreateContainer,
    CreateContainerComponent, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateTextDisplay, MessageFlags,
};
use serenity::prelude::Context;

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("help").description("Список команд бота")
}

fn signature(command: &Command) -> String {
    let mut text = format!("/{}", command.name);
    for option in &command.options {
        let (open, close) = if option.required() {
            ('<', '>')
        } else {
            ('[', ']')
        };
        text.push_str(&format!(" {open}{}{close}", option.name));
    }
    text
}

fn build_container(commands: Option<&[Command]>) -> CreateContainer<'static> {
    let header = CreateTextDisplay::new("`/commands/help`");
    let body = match commands {
        Some(commands) => CreateTextDisplay::new(
            commands
                .iter()
                .map(|command| format!("`{}` - {}", signature(command), command.description))
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        None => CreateTextDisplay::new("не удалось получить список команд"),
    };

    CreateContainer::new(vec![
        CreateContainerComponent::TextDisplay(header),
        CreateContainerComponent::TextDisplay(body),
    ])
}

async fn fetch_commands(ctx: &Context) -> Option<Vec<Command>> {
    match Command::get_global_commands(&ctx.http).await {
        Ok(mut commands) => {
            commands.sort_by(|left, right| left.name.cmp(&right.name));
            Some(commands)
        }
        Err(why) => {
            eprintln!("не удалось получить список команд: {why}");
            None
        }
    }
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let commands = fetch_commands(ctx).await;
    let container = build_container(commands.as_deref());
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

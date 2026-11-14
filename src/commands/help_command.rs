// Copyright (c) 2026 Greg, l1acy

use std::collections::{HashMap, HashSet};

use serenity::all::{
    Command, CommandInteraction, CreateCommand, CreateComponent, CreateContainer,
    CreateContainerComponent, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateSeparator, CreateTextDisplay, MessageFlags,
};
use serenity::prelude::Context;

use super::registry;

pub const NAME: &str = "help";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME).description("Список команд бота")
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

struct Block {
    title: &'static str,
    lines: Vec<String>,
}

fn command_line(command: &Command) -> String {
    format!("`{}` - {}", signature(command), command.description)
}

fn build_container(commands: Option<&[Command]>) -> CreateContainer<'static> {
    let header = CreateTextDisplay::new("`/commands/help`");

    let Some(commands) = commands else {
        return CreateContainer::new(vec![
            CreateContainerComponent::TextDisplay(header),
            CreateContainerComponent::TextDisplay(CreateTextDisplay::new(
                "не удалось получить список команд",
            )),
        ]);
    };

    let by_name: HashMap<&str, &Command> = commands
        .iter()
        .map(|command| (command.name.as_str(), command))
        .collect();

    let mut blocks: Vec<Block> = Vec::new();
    let mut title = "";
    let mut lines = Vec::new();

    for entry in registry::ALL {
        match entry {
            registry::Entry::Group(name) => {
                if !lines.is_empty() {
                    blocks.push(Block {
                        title,
                        lines: std::mem::take(&mut lines),
                    });
                }
                title = name;
            }
            registry::Entry::Command(entry) => lines.push(match by_name.get(entry.name) {
                Some(command) => command_line(command),
                None => format!("`/{}`", entry.name),
            }),
        }
    }

    if !lines.is_empty() {
        blocks.push(Block { title, lines });
    }

    let known: HashSet<&str> = registry::ALL
        .iter()
        .filter_map(|entry| match entry {
            registry::Entry::Command(entry) => Some(entry.name),
            registry::Entry::Group(_) => None,
        })
        .collect();

    let leftovers: Vec<String> = commands
        .iter()
        .filter(|command| !known.contains(command.name.as_str()))
        .map(command_line)
        .collect();

    if !leftovers.is_empty() {
        blocks.push(Block {
            title: "прочее",
            lines: leftovers,
        });
    }

    let mut components = vec![CreateContainerComponent::TextDisplay(header)];
    for (index, block) in blocks.into_iter().enumerate() {
        if index > 0 {
            components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
        }

        let text = if block.title.is_empty() {
            block.lines.join("\n")
        } else {
            format!("**{}:**\n{}", block.title, block.lines.join("\n"))
        };
        components.push(CreateContainerComponent::TextDisplay(
            CreateTextDisplay::new(text),
        ));
    }

    CreateContainer::new(components)
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

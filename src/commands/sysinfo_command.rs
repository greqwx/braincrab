// Copyright (c) 2026 Greg, l1acy

use serenity::all::{
    CommandInteraction, CreateCommand, CreateComponent, CreateContainer, CreateContainerComponent,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateSeparator,
    CreateTextDisplay, MessageFlags,
};
use serenity::prelude::Context;
use sysinfo::{Disks, System};

pub const NAME: &str = "sysinfo";

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new(NAME).description("Показать состояние системы, где работает бот")
}

fn human_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = seconds % 86_400 / 3_600;
    let minutes = seconds % 3_600 / 60;

    match (days, hours) {
        (0, 0) => format!("{minutes} мин"),
        (0, _) => format!("{hours} ч {minutes} мин"),
        _ => format!("{days} д {hours} ч"),
    }
}

fn gib(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

struct SysinfoSections {
    base: String,
    resources: String,
    uptime: String,
}

fn build_sections(system: &System, disks: Option<&Disks>) -> SysinfoSections {
    let os = System::long_os_version()
        .ok()
        .or_else(|| System::os_version().ok())
        .unwrap_or_else(|| "неизвестно".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|_| "неизвестно".to_string());
    let host = System::host_name().unwrap_or_else(|_| "неизвестно".to_string());

    let cpu = system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim().to_string())
        .unwrap_or_else(|| "неизвестно".to_string());

    let disk = match disks {
        Some(disks) => {
            let total: u64 = disks.iter().map(|disk| disk.total_space()).sum();
            let free: u64 = disks.iter().map(|disk| disk.available_space()).sum();
            format!("{:.1} / {:.1} GiB", gib(total - free), gib(total))
        }
        None => "неизвестно".to_string(),
    };

    let bot_uptime = sysinfo::get_current_pid()
        .ok()
        .and_then(|pid| system.process(pid))
        .map(|process| process.run_time())
        .unwrap_or(0);

    let base = [
        format!("ос: `{os}`"),
        format!("ядро: `{kernel}`"),
        format!("хост: `{host}`"),
    ]
    .join("\n");

    let resources = [
        format!("cpu: `{cpu}`"),
        format!("диск: `{disk}`"),
        format!(
            "память: `{:.1} / {:.1}` GiB",
            gib(system.used_memory()),
            gib(system.total_memory()),
        ),
    ]
    .join("\n");

    let uptime = [
        format!(
            "аптайм ос: `{}`",
            human_uptime(System::uptime().unwrap_or(0))
        ),
        format!("аптайм бота: `{}`", human_uptime(bot_uptime)),
    ]
    .join("\n");

    SysinfoSections {
        base,
        resources,
        uptime,
    }
}

fn build_container(system: Option<&System>, disks: Option<&Disks>) -> CreateContainer<'static> {
    let header = CreateTextDisplay::new("`/commands/sysinfo`");

    let mut components = vec![CreateContainerComponent::TextDisplay(header)];

    match system {
        Some(system) => {
            let sections = build_sections(system, disks);

            components.push(CreateContainerComponent::TextDisplay(
                CreateTextDisplay::new(sections.base),
            ));
            components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
            components.push(CreateContainerComponent::TextDisplay(
                CreateTextDisplay::new(sections.resources),
            ));
            components.push(CreateContainerComponent::Separator(CreateSeparator::new()));
            components.push(CreateContainerComponent::TextDisplay(
                CreateTextDisplay::new(sections.uptime),
            ));
        }
        None => {
            components.push(CreateContainerComponent::TextDisplay(
                CreateTextDisplay::new("не удалось собрать данные о системе"),
            ));
        }
    }

    CreateContainer::new(components)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let system = match System::new_all() {
        Ok(system) => Some(system),
        Err(why) => {
            eprintln!("не удалось собрать данные о системе: {why}");
            None
        }
    };
    let disks = Disks::new_with_refreshed_list().ok();

    let container = build_container(system.as_ref(), disks.as_ref());
    let response = CreateInteractionResponseMessage::new()
        .components(vec![CreateComponent::Container(container)])
        .flags(MessageFlags::IS_COMPONENTS_V2);

    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

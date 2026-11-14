// Copyright (c) 2026 Greg, l1acy

use serenity::all::CreateCommand;

use super::{
    avatar_command, bot_command, help_command, ping_command, purge_command, sysinfo_command,
    userinfo_command,
};

pub struct CommandEntry {
    pub name: &'static str,
    pub register: fn() -> CreateCommand<'static>,
}

pub enum Entry {
    /// заголовок категории
    Group(&'static str),
    Command(CommandEntry),
}

pub const ALL: &[Entry] = &[
    Entry::Group("базовое"),
    Entry::Command(CommandEntry {
        name: avatar_command::NAME,
        register: avatar_command::register,
    }),
    Entry::Command(CommandEntry {
        name: bot_command::NAME,
        register: bot_command::register,
    }),
    Entry::Command(CommandEntry {
        name: help_command::NAME,
        register: help_command::register,
    }),
    Entry::Group("информация"),
    Entry::Command(CommandEntry {
        name: ping_command::NAME,
        register: ping_command::register,
    }),
    Entry::Command(CommandEntry {
        name: sysinfo_command::NAME,
        register: sysinfo_command::register,
    }),
    Entry::Command(CommandEntry {
        name: userinfo_command::NAME,
        register: userinfo_command::register,
    }),
    Entry::Group("модерация"),
    Entry::Command(CommandEntry {
        name: purge_command::NAME,
        register: purge_command::register,
    }),
];

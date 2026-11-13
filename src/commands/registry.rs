// Copyright (c) 2026 Greg, l1acy

use serenity::all::CreateCommand;

use super::{avatar_command, bot_command, help_command, ping_command};

pub const ALL: &[fn() -> CreateCommand<'static>] = &[
    avatar_command::register,
    bot_command::register,
    help_command::register,
    ping_command::register,
];

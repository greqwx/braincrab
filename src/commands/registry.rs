// Copyright (c) 2026 Greg, l1acy

use serenity::all::CreateCommand;

use super::{
    avatar_command, bot_command, help_command, ping_command, purge_command, sysinfo_command,
    userinfo_command,
};

pub const ALL: &[fn() -> CreateCommand<'static>] = &[
    avatar_command::register,
    bot_command::register,
    help_command::register,
    ping_command::register,
    purge_command::register,
    sysinfo_command::register,
    userinfo_command::register,
];

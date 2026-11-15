// Copyright (c) 2026 Greg, l1acy

use serenity::all::{ChannelId, GuildId, RoleId};

/// константы которые можно и НУЖНО менять при локальном развертывании
/// если не глупый сам поймешь думаю где какие
pub const MODERATOR_ROLES: &[RoleId] = &[RoleId::new(1571150229200437329)];

/// сервер, события которого пишутся в аудит
pub const AUDIT_GUILD_ID: GuildId = GuildId::new(1569027866912620728);

/// канал, куда складываются записи аудита
pub const AUDIT_LOG_CHANNEL: ChannelId = ChannelId::new(1571426228437786694);

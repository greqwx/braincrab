// Copyright (c) 2026 Greg, l1acy

use serenity::all::{GuildId, GuildMemberUpdateEvent, Member, RoleId, User};
use serenity::prelude::Context;

use super::render::{role, timestamp, user};

pub fn joined(member: &Member) -> Option<(GuildId, String)> {
    let mut lines = vec![
        "**участник присоединился**".to_owned(),
        account_line(&member.user),
        format!("аккаунт создан {}", timestamp(member.user.id.created_at())),
    ];

    if let Some(joined_at) = member.joined_at {
        lines.push(format!("на сервере с {}", timestamp(joined_at)));
    }

    Some((member.guild_id, lines.join("\n")))
}

pub fn left(
    ctx: &Context,
    guild_id: GuildId,
    who: &User,
    member: Option<&Member>,
) -> Option<(GuildId, String)> {
    let mut lines = vec!["**участник покинул сервер**".to_owned(), account_line(who)];

    if let Some(member) = member {
        if let Some(joined_at) = member.joined_at {
            lines.push(format!("был на сервере с {}", timestamp(joined_at)));
        }
        if let Some(list) = role_list(ctx, guild_id, &member.roles) {
            lines.push(format!("роли: {list}"));
        }
    }

    Some((guild_id, lines.join("\n")))
}

pub fn updated(
    ctx: &Context,
    old: Option<&Member>,
    event: &GuildMemberUpdateEvent,
) -> Option<(GuildId, String)> {
    let old = old?;
    let mut changes = Vec::new();

    let old_nick = old.nick.as_deref();
    let new_nick = event.nick.as_deref();
    if old_nick != new_nick {
        changes.push(format!(
            "ник: `{}` → `{}`",
            old_nick.unwrap_or_default(),
            new_nick.unwrap_or_default()
        ));
    }

    let new_roles = event.roles.iter().copied().collect::<Vec<_>>();
    let added = new_roles
        .iter()
        .filter(|id| !old.roles.contains(id))
        .copied()
        .collect::<Vec<_>>();
    let removed = old
        .roles
        .iter()
        .filter(|id| !new_roles.contains(id))
        .copied()
        .collect::<Vec<_>>();

    if let Some(list) = role_list(ctx, event.guild_id, &added) {
        changes.push(format!("выданы роли: {list}"));
    }
    if let Some(list) = role_list(ctx, event.guild_id, &removed) {
        changes.push(format!("сняты роли: {list}"));
    }

    if old.communication_disabled_until != event.communication_disabled_until {
        changes.push(match event.communication_disabled_until {
            Some(until) => format!("тайм-аут до {}", timestamp(until)),
            None => "тайм-аут снят".to_owned(),
        });
    }

    if changes.is_empty() {
        return None;
    }

    let mut lines = vec!["**участник изменён**".to_owned(), account_line(&event.user)];
    lines.extend(changes);

    Some((event.guild_id, lines.join("\n")))
}

fn account_line(account: &User) -> String {
    format!("{} (`{}`)", user(account.id), account.name)
}

fn role_list(ctx: &Context, guild_id: GuildId, roles: &[RoleId]) -> Option<String> {
    if roles.is_empty() {
        return None;
    }

    let names = roles
        .iter()
        .map(|id| role_name(ctx, guild_id, *id))
        .collect::<Vec<_>>()
        .join(", ");

    Some(names)
}

fn role_name(ctx: &Context, guild_id: GuildId, role_id: RoleId) -> String {
    ctx.cache
        .guild(guild_id)
        .and_then(|guild| {
            guild
                .roles
                .get(&role_id)
                .map(|role| role.name.to_string())
        })
        .unwrap_or_else(|| role(role_id))
}

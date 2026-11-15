// Copyright (c) 2026 Greg, l1acy

use serenity::all::{GuildId, VoiceState};

use super::render::{channel, user};

/// логируем только смену канала, mute и deaf оставляем в покое
pub fn changed(old: Option<&VoiceState>, new: &VoiceState) -> Option<(GuildId, String)> {
    let old = old?;
    let guild_id = new.guild_id?;

    let who = match &new.member {
        Some(member) => format!("{} (`{}`)", user(member.user.id), member.display_name()),
        None => user(new.user_id),
    };

    let mut lines = Vec::new();
    match (old.channel_id, new.channel_id) {
        (Some(before), Some(after)) if before != after => {
            lines.push("**переместился между каналами**".to_owned());
            lines.push(who);
            lines.push(format!("{} → {}", channel(before), channel(after)));
        }
        (None, Some(after)) => {
            lines.push("**вошёл в голосовой канал**".to_owned());
            lines.push(who);
            lines.push(format!("канал {}", channel(after)));
        }
        (Some(before), None) => {
            lines.push("**вышел из голосового канала**".to_owned());
            lines.push(who);
            lines.push(format!("был в {}", channel(before)));
        }
        _ => return None,
    }

    Some((guild_id, lines.join("\n")))
}

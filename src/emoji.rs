//! Central registry for Moonbyte's Discord custom emojis.
//!
//! Upload the PNGs to Discord, then replace each `None` below with the
//! numeric custom-emoji ID. Do not paste the surrounding Discord markup.
//!
//! Example:
//! `pub const MOONBYTE: Option<&str> = Some("123456789012345678");`

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmojiKey {
    Moonbyte,
    Rocket,
    Treasure,
    Crystal,
    Chat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmojiSlot {
    pub key: EmojiKey,
    pub name: &'static str,
    pub id: Option<&'static str>,
}

pub mod ids {
    /// Upload name: `moonbyte`.
    pub const MOONBYTE: Option<&str> = None;
    /// Upload name: `moonbyte_rocket`.
    pub const ROCKET: Option<&str> = None;
    /// Upload name: `moonbyte_treasure`.
    pub const TREASURE: Option<&str> = None;
    /// Upload name: `moonbyte_crystal`.
    pub const CRYSTAL: Option<&str> = None;
    /// Upload name: `moonbyte_chat`.
    pub const CHAT: Option<&str> = None;
}

pub const EMOJI_SLOTS: [EmojiSlot; 5] = [
    EmojiSlot {
        key: EmojiKey::Moonbyte,
        name: "moonbyte",
        id: ids::MOONBYTE,
    },
    EmojiSlot {
        key: EmojiKey::Rocket,
        name: "moonbyte_rocket",
        id: ids::ROCKET,
    },
    EmojiSlot {
        key: EmojiKey::Treasure,
        name: "moonbyte_treasure",
        id: ids::TREASURE,
    },
    EmojiSlot {
        key: EmojiKey::Crystal,
        name: "moonbyte_crystal",
        id: ids::CRYSTAL,
    },
    EmojiSlot {
        key: EmojiKey::Chat,
        name: "moonbyte_chat",
        id: ids::CHAT,
    },
];

pub const fn slot(key: EmojiKey) -> EmojiSlot {
    match key {
        EmojiKey::Moonbyte => EMOJI_SLOTS[0],
        EmojiKey::Rocket => EMOJI_SLOTS[1],
        EmojiKey::Treasure => EMOJI_SLOTS[2],
        EmojiKey::Crystal => EMOJI_SLOTS[3],
        EmojiKey::Chat => EMOJI_SLOTS[4],
    }
}

/// Build Discord's static custom-emoji syntax from a known name and numeric ID.
/// Invalid or missing IDs return None so callers can use a Unicode fallback.
pub fn custom_mention(name: &str, id: Option<&str>) -> Option<String> {
    let id = id?;
    let valid_name = !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_');
    let valid_id = !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit());

    if !valid_name || !valid_id {
        return None;
    }

    Some(format!("<:{name}:{id}>"))
}

pub fn mention(key: EmojiKey) -> Option<String> {
    let emoji = slot(key);
    custom_mention(emoji.name, emoji.id)
}

/// Return the uploaded custom emoji when configured, otherwise a Unicode fallback.
pub fn render(key: EmojiKey, fallback: &str) -> String {
    mention(key).unwrap_or_else(|| fallback.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{EMOJI_SLOTS, EmojiKey, custom_mention, mention, render};

    #[test]
    fn every_uploaded_name_has_a_paste_in_slot() {
        let names = EMOJI_SLOTS.map(|slot| slot.name);
        assert_eq!(
            names,
            [
                "moonbyte",
                "moonbyte_rocket",
                "moonbyte_treasure",
                "moonbyte_crystal",
                "moonbyte_chat"
            ]
        );
        assert!(EMOJI_SLOTS.iter().all(|slot| slot.id.is_none()));
    }

    #[test]
    fn custom_emoji_markup_uses_the_numeric_id() {
        assert_eq!(
            custom_mention("moonbyte", Some("123456789012345678")).as_deref(),
            Some("<:moonbyte:123456789012345678>")
        );
    }

    #[test]
    fn invalid_or_missing_ids_fall_back_without_broken_markup() {
        assert_eq!(custom_mention("moonbyte", None), None);
        assert_eq!(custom_mention("moonbyte", Some("not-an-id")), None);
        assert_eq!(custom_mention("moonbyte;spoof", Some("123")), None);
        assert_eq!(mention(EmojiKey::Moonbyte), None);
        assert_eq!(render(EmojiKey::Crystal, "💎"), "💎");
    }
}

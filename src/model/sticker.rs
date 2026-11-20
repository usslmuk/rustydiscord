use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    id::{GenericId, GuildId, SkuId, StickerId, StickerPackId},
    macros::int_enum,
    user::User,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Sticker {
    pub id: StickerId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_id: Option<StickerPackId>,

    pub name: CompactString,

    pub description: Option<CompactString>,

    pub tags: CompactString,

    #[serde(rename = "type")]
    pub kind: StickerType,

    pub format_type: StickerFormatType,

    #[serde(default = "crate::model::serde_util::default_true")]
    pub available: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<u32>,
}

int_enum! {



    pub enum StickerType: u8 {

        Standard = 1,

        Guild = 2,
    }
}

int_enum! {



    pub enum StickerFormatType: u8 {

        Png = 1,

        Apng = 2,

        Lottie = 3,

        Gif = 4,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StickerItem {
    pub id: StickerId,

    pub name: CompactString,

    pub format_type: StickerFormatType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StickerPack {
    pub id: StickerPackId,

    pub stickers: Vec<Sticker>,

    pub name: CompactString,

    pub sku_id: SkuId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_sticker_id: Option<StickerId>,

    pub description: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_asset_id: Option<GenericId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_sticker_is_available() {
        let json = r#"{"id":"749054660769218631","name":"Wave","tags":"wumpus, hello, sup, hi, oi, heyo, heya, yo, wave",
            "type":1,"format_type":3,"description":"Wumpus waves hello","asset":"",
            "pack_id":"847199849233514549","sort_value":12}"#;
        let sticker: Sticker = serde_json::from_str(json).unwrap();
        assert_eq!(sticker.kind, StickerType::Standard);
        assert_eq!(sticker.format_type, StickerFormatType::Lottie);
        assert!(sticker.available);
        assert_eq!(sticker.sort_value, Some(12));
    }

    #[test]
    fn sticker_item() {
        let item: StickerItem =
            serde_json::from_str(r#"{"id":"1","name":"x","format_type":4}"#).unwrap();
        assert_eq!(item.format_type, StickerFormatType::Gif);
    }
}

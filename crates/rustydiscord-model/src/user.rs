use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    guild::PartialIntegration,
    id::{GuildId, SkuId, UserId},
    macros::{int_enum, int_flags, str_enum},
    util::{ImageHash, Locale},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    pub id: UserId,

    pub username: CompactString,

    #[serde(with = "discriminator")]
    pub discriminator: u16,

    pub global_name: Option<CompactString>,

    pub avatar: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub bot: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub system: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Box<AvatarDecorationData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<Collectibles>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_guild: Option<Box<PrimaryGuild>>,
}

impl User {
    #[must_use]
    pub fn display_name(&self) -> &str {
        display_name(&self.username, self.global_name.as_deref())
    }

    #[must_use]
    pub fn tag(&self) -> CompactString {
        tag(&self.username, self.discriminator)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CurrentUser {
    pub id: UserId,

    pub username: CompactString,

    #[serde(with = "discriminator")]
    pub discriminator: u16,

    pub global_name: Option<CompactString>,

    pub avatar: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub bot: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub system: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub mfa_enabled: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<Locale>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub verified: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Box<AvatarDecorationData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<Collectibles>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_guild: Option<Box<PrimaryGuild>>,
}

impl CurrentUser {
    #[must_use]
    pub fn display_name(&self) -> &str {
        display_name(&self.username, self.global_name.as_deref())
    }

    #[must_use]
    pub fn tag(&self) -> CompactString {
        tag(&self.username, self.discriminator)
    }
}

impl From<CurrentUser> for User {
    fn from(user: CurrentUser) -> Self {
        Self {
            id: user.id,
            username: user.username,
            discriminator: user.discriminator,
            global_name: user.global_name,
            avatar: user.avatar,
            bot: user.bot,
            system: user.system,
            banner: user.banner,
            accent_color: user.accent_color,
            public_flags: user.public_flags,
            avatar_decoration_data: user.avatar_decoration_data,
            collectibles: user.collectibles,
            primary_guild: user.primary_guild,
        }
    }
}

fn display_name<'a>(username: &'a str, global_name: Option<&'a str>) -> &'a str {
    global_name.unwrap_or(username)
}

fn tag(username: &str, discriminator: u16) -> CompactString {
    if discriminator == 0 {
        username.into()
    } else {
        compact_str::format_compact!("{username}#{discriminator:04}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvatarDecorationData {
    pub asset: ImageHash,

    pub sku_id: SkuId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Collectibles {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nameplate: Option<Nameplate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Nameplate {
    pub sku_id: SkuId,

    pub asset: CompactString,

    pub label: CompactString,

    pub palette: NameplatePalette,
}

str_enum! {



    pub enum NameplatePalette {

        Crimson = "crimson",

        Berry = "berry",

        Sky = "sky",

        Teal = "teal",

        Forest = "forest",

        BubbleGum = "bubble_gum",

        Violet = "violet",

        Cobalt = "cobalt",




        Clover = "clover",




        Lemon = "lemon",




        White = "white",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PrimaryGuild {
    pub identity_guild_id: Option<GuildId>,

    pub identity_enabled: Option<bool>,

    pub tag: Option<CompactString>,

    pub badge: Option<ImageHash>,
}

int_enum! {




    pub enum PremiumType: u8 {

        None = 0,

        NitroClassic = 1,

        Nitro = 2,

        NitroBasic = 3,
    }
}

pub(crate) mod option_discriminator {
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Deserialize)]
    struct Discriminator(#[serde(with = "super::discriminator")] u16);

    #[expect(
        clippy::ref_option,
        clippy::trivially_copy_pass_by_ref,
        reason = "signature required by serde(with)"
    )]
    pub(crate) fn serialize<S: Serializer>(
        value: &Option<u16>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => super::discriminator::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<u16>, D::Error> {
        Option::<Discriminator>::deserialize(deserializer).map(|value| value.map(|value| value.0))
    }
}

pub(crate) mod discriminator {
    use core::fmt;

    use serde::{
        Deserializer, Serializer,
        de::{Error, Unexpected, Visitor},
    };

    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "signature required by serde(with)"
    )]
    pub(crate) fn serialize<S: Serializer>(value: &u16, serializer: S) -> Result<S::Ok, S::Error> {
        if *value == 0 {
            serializer.serialize_str("0")
        } else {
            serializer.collect_str(&format_args!("{value:04}"))
        }
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u16, D::Error> {
        struct DiscriminatorVisitor;

        impl Visitor<'_> for DiscriminatorVisitor {
            type Value = u16;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a discriminator of up to four digits")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<u16, E> {
                if value.is_empty() || value.len() > 4 || !value.bytes().all(|b| b.is_ascii_digit())
                {
                    return Err(E::invalid_value(Unexpected::Str(value), &self));
                }
                value
                    .parse()
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
            }

            fn visit_u64<E: Error>(self, value: u64) -> Result<u16, E> {
                u16::try_from(value)
                    .ok()
                    .filter(|value| *value <= 9999)
                    .ok_or_else(|| E::invalid_value(Unexpected::Unsigned(value), &self))
            }
        }

        deserializer.deserialize_any(DiscriminatorVisitor)
    }
}

int_flags! {
















    pub struct UserFlags: u64 {

        const STAFF = 1 << 0;

        const PARTNER = 1 << 1;

        const HYPESQUAD = 1 << 2;

        const BUG_HUNTER_LEVEL_1 = 1 << 3;




        const MFA_SMS = 1 << 4;




        const PREMIUM_PROMO_DISMISSED = 1 << 5;

        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;

        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;

        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;

        const PREMIUM_EARLY_SUPPORTER = 1 << 9;

        const TEAM_PSEUDO_USER = 1 << 10;




        const HAS_UNREAD_URGENT_MESSAGES = 1 << 13;

        const BUG_HUNTER_LEVEL_2 = 1 << 14;

        const VERIFIED_BOT = 1 << 16;

        const VERIFIED_DEVELOPER = 1 << 17;

        const CERTIFIED_MODERATOR = 1 << 18;

        const BOT_HTTP_INTERACTIONS = 1 << 19;




        const SPAMMER = 1 << 20;




        const DISABLE_PREMIUM = 1 << 21;






        const ACTIVE_DEVELOPER = 1 << 22;




        const QUARANTINED = 1 << 44;




        const COLLABORATOR = 1 << 50;




        const RESTRICTED_COLLABORATOR = 1 << 51;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Connection {
    pub id: CompactString,

    pub name: CompactString,

    #[serde(rename = "type")]
    pub kind: ConnectionService,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub revoked: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub integrations: Vec<PartialIntegration>,

    pub verified: bool,

    pub friend_sync: bool,

    pub show_activity: bool,

    pub two_way_link: bool,

    pub visibility: ConnectionVisibility,
}

str_enum! {





    #[doc(alias = "Services")]
    pub enum ConnectionService {

        AmazonMusic = "amazon-music",



        BattleNet = "battlenet",

        Bluesky = "bluesky",

        BungieNet = "bungie",

        Crunchyroll = "crunchyroll",

        Domain = "domain",

        Ebay = "ebay",

        EpicGames = "epicgames",

        Facebook = "facebook",

        GitHub = "github",

        Instagram = "instagram",



        LeagueOfLegends = "leagueoflegends",

        Mastodon = "mastodon",

        PayPal = "paypal",

        PlayStationNetwork = "playstation",

        Reddit = "reddit",



        RiotGames = "riotgames",

        Roblox = "roblox",

        Spotify = "spotify",

        Skype = "skype",

        Steam = "steam",

        TikTok = "tiktok",

        Twitch = "twitch",

        #[doc(alias = "Twitter")]
        X = "twitter",

        Xbox = "xbox",

        YouTube = "youtube",
    }
}

int_enum! {






    pub enum ConnectionVisibility: u8 {

        None = 0,

        Everyone = 1,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationRoleConnection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_username: Option<CompactString>,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub metadata: BTreeMap<CompactString, CompactString>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCS_EXAMPLE: &str = r#"{
        "id": "80351110224678912",
        "username": "Nelly",
        "global_name": null,
        "discriminator": "1337",
        "avatar": "8342729096ea3675442027381ff50dfe",
        "verified": true,
        "email": "nelly@discord.com",
        "flags": 64,
        "banner": "06c16474723fe537c283b8efa61a30c8",
        "accent_color": 16711680,
        "premium_type": 0,
        "public_flags": 64,
        "avatar_decoration_data": {
            "sku_id": "1144058844004233369",
            "asset": "a_fed43ab12698df65902ba06727e20c0e"
        },
        "collectibles": {
            "nameplate": {
                "sku_id": "2247558840304243311",
                "asset": "nameplates/nameplates/twilight/",
                "label": "",
                "palette": "cobalt"
            }
        },
        "primary_guild": {
            "identity_guild_id": "1234647491267808778",
            "identity_enabled": true,
            "tag": "DISC",
            "badge": "7d1734ae5a615e82bc7a4033b98fade8"
        }
    }"#;

    #[test]
    fn deserialize_documented_example_as_user() {
        let user: User = serde_json::from_str(DOCS_EXAMPLE).unwrap();
        assert_eq!(user.id.get(), 80_351_110_224_678_912);
        assert_eq!(user.username, "Nelly");
        assert_eq!(user.discriminator, 1337);
        assert_eq!(user.global_name, None);
        assert_eq!(
            user.avatar.unwrap().to_string(),
            "8342729096ea3675442027381ff50dfe"
        );
        assert!(!user.bot);
        assert_eq!(user.accent_color, Some(0xFF_0000));
        assert_eq!(user.public_flags, Some(UserFlags::HYPESQUAD_ONLINE_HOUSE_1));
        let decoration = user.avatar_decoration_data.as_ref().unwrap();
        assert_eq!(decoration.sku_id.get(), 1_144_058_844_004_233_369);
        assert!(decoration.asset.is_animated());
        let nameplate = user
            .collectibles
            .as_ref()
            .unwrap()
            .nameplate
            .as_ref()
            .unwrap();
        assert_eq!(nameplate.palette, NameplatePalette::Cobalt);
        assert_eq!(nameplate.asset, "nameplates/nameplates/twilight/");
        let guild = user.primary_guild.as_ref().unwrap();
        assert_eq!(guild.tag.as_deref(), Some("DISC"));
        assert_eq!(guild.identity_enabled, Some(true));
        assert_eq!(user.tag(), "Nelly#1337");
        assert_eq!(user.display_name(), "Nelly");
    }

    #[test]
    fn deserialize_documented_example_as_current_user() {
        let user: CurrentUser = serde_json::from_str(DOCS_EXAMPLE).unwrap();
        assert!(user.verified);
        assert_eq!(user.email.as_deref(), Some("nelly@discord.com"));
        assert_eq!(user.flags, Some(UserFlags::HYPESQUAD_ONLINE_HOUSE_1));
        assert_eq!(user.premium_type, Some(PremiumType::None));
        assert!(!user.mfa_enabled);
        assert_eq!(user.locale, None);
        let plain = User::from(user.clone());
        assert_eq!(plain, serde_json::from_str::<User>(DOCS_EXAMPLE).unwrap());
    }

    #[test]
    fn serialization_keeps_rustydiscord_shape() {
        let json = r#"{"id":"1","username":"bot","discriminator":"0007","global_name":"Bot","avatar":null,"bot":true}"#;
        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(serde_json::to_string(&user).unwrap(), json);
    }

    #[test]
    fn missing_nullable_fields_are_accepted() {
        let user: User =
            serde_json::from_str(r#"{"id":"1","username":"a","discriminator":"0"}"#).unwrap();
        assert_eq!(user.global_name, None);
        assert_eq!(user.avatar, None);
    }

    #[test]
    fn migrated_user_discriminator() {
        let json =
            r#"{"id":"1","username":"a","discriminator":"0","global_name":null,"avatar":null}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.discriminator, 0);
        assert!(
            serde_json::to_string(&user)
                .unwrap()
                .contains(r#""discriminator":"0""#)
        );
    }

    #[test]
    fn rejects_bad_discriminator() {
        for bad in [r#""12345""#, r#""""#, r#""12a""#] {
            let json =
                format!(r#"{{"id":"1","username":"a","discriminator":{bad},"avatar":null}}"#);
            assert!(
                serde_json::from_str::<User>(&json).is_err(),
                "{bad} should be rejected"
            );
        }
    }

    #[test]
    fn hot_type_sizes() {
        assert!(
            size_of::<User>() <= 256,
            "User is {} bytes",
            size_of::<User>()
        );
    }
}

use core::fmt::{self, Write as _};

use crate::model::{
    id::{
        ApplicationId, ChannelId, EmojiId, GenericId, GuildId, RoleId, ScheduledEventId,
        SoundboardSoundId, StickerId, TeamId, UserId,
    },
    sticker::StickerFormatType,
    util::ImageHash,
};

pub const DEFAULT_CDN: &str = "https://cdn.discordapp.com";

pub const DEFAULT_MEDIA_PROXY: &str = "https://media.discordapp.net";

const STICKER_PACK_APPLICATION: u64 = 710_982_414_301_790_216;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ImageFormat {
    Webp,

    Png,

    Jpeg,

    Gif,
}

impl ImageFormat {
    #[must_use]
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Webp => "webp",
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Gif => "gif",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ImageSize {
    S16,

    S32,

    S64,

    S128,

    S256,

    S512,

    S1024,

    S2048,

    S4096,
}

impl ImageSize {
    #[must_use]
    pub const fn pixels(self) -> u16 {
        match self {
            Self::S16 => 16,
            Self::S32 => 32,
            Self::S64 => 64,
            Self::S128 => 128,
            Self::S256 => 256,
            Self::S512 => 512,
            Self::S1024 => 1024,
            Self::S2048 => 2048,
            Self::S4096 => 4096,
        }
    }

    #[must_use]
    pub const fn from_pixels(pixels: u16) -> Option<Self> {
        Some(match pixels {
            16 => Self::S16,
            32 => Self::S32,
            64 => Self::S64,
            128 => Self::S128,
            256 => Self::S256,
            512 => Self::S512,
            1024 => Self::S1024,
            2048 => Self::S2048,
            4096 => Self::S4096,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
pub struct ImageOptions {
    pub format: Option<ImageFormat>,

    pub size: Option<ImageSize>,

    pub force_static: bool,
}

impl ImageOptions {
    pub const fn new() -> Self {
        Self {
            format: None,
            size: None,
            force_static: false,
        }
    }

    pub const fn format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub const fn size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    pub const fn force_static(mut self, force_static: bool) -> Self {
        self.force_static = force_static;
        self
    }
}

#[must_use]
pub const fn default_avatar_index(user_id: UserId, discriminator: u16) -> u8 {
    if discriminator == 0 {
        ((user_id.get() >> 22) % 6) as u8
    } else {
        (discriminator % 5) as u8
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cdn {
    cdn: String,
    media_proxy: String,
}

impl Default for Cdn {
    fn default() -> Self {
        Self::new(DEFAULT_CDN, DEFAULT_MEDIA_PROXY)
    }
}

impl Cdn {
    #[must_use]
    pub fn new(cdn: impl Into<String>, media_proxy: impl Into<String>) -> Self {
        let trim = |mut url: String| {
            let len = url.trim_end_matches('/').len();
            url.truncate(len);
            url
        };
        Self {
            cdn: trim(cdn.into()),
            media_proxy: trim(media_proxy.into()),
        }
    }

    #[must_use]
    pub fn base(&self) -> &str {
        &self.cdn
    }

    #[must_use]
    pub fn media_proxy(&self) -> &str {
        &self.media_proxy
    }

    #[must_use]
    pub fn emoji(&self, emoji_id: EmojiId, animated: bool, options: ImageOptions) -> String {
        self.image(format_args!("emojis/{emoji_id}"), animated, options)
    }

    #[must_use]
    pub fn guild_icon(&self, guild_id: GuildId, hash: ImageHash, options: ImageOptions) -> String {
        self.hashed(format_args!("icons/{guild_id}"), hash, options)
    }

    #[must_use]
    pub fn guild_splash(
        &self,
        guild_id: GuildId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("splashes/{guild_id}"), hash, options)
    }

    #[must_use]
    pub fn guild_discovery_splash(
        &self,
        guild_id: GuildId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("discovery-splashes/{guild_id}"), hash, options)
    }

    #[must_use]
    pub fn guild_banner(
        &self,
        guild_id: GuildId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("banners/{guild_id}"), hash, options)
    }

    #[must_use]
    pub fn user_banner(&self, user_id: UserId, hash: ImageHash, options: ImageOptions) -> String {
        self.hashed(format_args!("banners/{user_id}"), hash, options)
    }

    #[must_use]
    pub fn default_user_avatar(&self, index: u8) -> String {
        Self::fixed(&self.cdn, format_args!("embed/avatars/{index}"), "png")
    }

    #[must_use]
    pub fn user_avatar(&self, user_id: UserId, hash: ImageHash, options: ImageOptions) -> String {
        self.hashed(format_args!("avatars/{user_id}"), hash, options)
    }

    #[must_use]
    pub fn guild_member_avatar(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(
            format_args!("guilds/{guild_id}/users/{user_id}/avatars"),
            hash,
            options,
        )
    }

    #[must_use]
    pub fn guild_member_banner(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(
            format_args!("guilds/{guild_id}/users/{user_id}/banners"),
            hash,
            options,
        )
    }

    #[must_use]
    pub fn avatar_decoration(&self, asset: ImageHash) -> String {
        Self::fixed(
            &self.cdn,
            format_args!("avatar-decoration-presets/{asset}"),
            "png",
        )
    }

    #[must_use]
    pub fn application_icon(
        &self,
        application_id: ApplicationId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("app-icons/{application_id}"), hash, options)
    }

    #[must_use]
    pub fn application_cover(
        &self,
        application_id: ApplicationId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("app-icons/{application_id}"), hash, options)
    }

    #[must_use]
    pub fn application_asset(
        &self,
        application_id: ApplicationId,
        asset_id: &str,
        options: ImageOptions,
    ) -> String {
        self.image(
            format_args!("app-assets/{application_id}/{asset_id}"),
            false,
            options,
        )
    }

    #[must_use]
    pub fn achievement_icon(
        &self,
        application_id: ApplicationId,
        achievement_id: GenericId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(
            format_args!("app-assets/{application_id}/achievements/{achievement_id}/icons"),
            hash,
            options,
        )
    }

    #[must_use]
    pub fn store_page_asset(
        &self,
        application_id: ApplicationId,
        asset_id: GenericId,
        options: ImageOptions,
    ) -> String {
        self.image(
            format_args!("app-assets/{application_id}/store/{asset_id}"),
            false,
            options,
        )
    }

    #[must_use]
    pub fn sticker_pack_banner(&self, banner_asset_id: GenericId, options: ImageOptions) -> String {
        self.image(
            format_args!("app-assets/{STICKER_PACK_APPLICATION}/store/{banner_asset_id}"),
            false,
            options,
        )
    }

    #[must_use]
    pub fn team_icon(&self, team_id: TeamId, hash: ImageHash, options: ImageOptions) -> String {
        self.hashed(format_args!("team-icons/{team_id}"), hash, options)
    }

    #[must_use]
    pub fn sticker(&self, sticker_id: StickerId, format_type: StickerFormatType) -> String {
        let path = format_args!("stickers/{sticker_id}");
        match format_type {
            StickerFormatType::Gif => Self::fixed(&self.media_proxy, path, "gif"),
            StickerFormatType::Lottie => Self::fixed(&self.cdn, path, "json"),
            _ => Self::fixed(&self.cdn, path, "png"),
        }
    }

    #[must_use]
    pub fn role_icon(&self, role_id: RoleId, hash: ImageHash, options: ImageOptions) -> String {
        self.hashed(format_args!("role-icons/{role_id}"), hash, options)
    }

    #[must_use]
    pub fn guild_scheduled_event_cover(
        &self,
        event_id: ScheduledEventId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("guild-events/{event_id}"), hash, options)
    }

    #[must_use]
    pub fn guild_tag_badge(
        &self,
        guild_id: GuildId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("guild-tag-badges/{guild_id}"), hash, options)
    }

    #[must_use]
    pub fn channel_icon(
        &self,
        channel_id: ChannelId,
        hash: ImageHash,
        options: ImageOptions,
    ) -> String {
        self.hashed(format_args!("channel-icons/{channel_id}"), hash, options)
    }

    #[must_use]
    pub fn soundboard_sound(&self, sound_id: SoundboardSoundId) -> String {
        format!("{}/soundboard-sounds/{sound_id}", self.cdn)
    }

    #[must_use]
    pub fn guild_widget_json(&self, guild_id: GuildId) -> String {
        format!("{}/guilds/{guild_id}/widget.json", self.cdn)
    }

    #[must_use]
    pub fn guild_widget_png(&self, guild_id: GuildId) -> String {
        format!("{}/guilds/{guild_id}/widget.png", self.cdn)
    }

    fn hashed(&self, path: fmt::Arguments<'_>, hash: ImageHash, options: ImageOptions) -> String {
        self.image(format_args!("{path}/{hash}"), hash.is_animated(), options)
    }

    fn image(&self, path: fmt::Arguments<'_>, animated: bool, options: ImageOptions) -> String {
        let animated = animated && !options.force_static;
        let format = options.format.unwrap_or(ImageFormat::Webp);
        let mut url = String::with_capacity(self.cdn.len() + 96);

        let _ = write!(url, "{}/{path}.{}", self.cdn, format.extension());
        let mut separator = '?';
        if animated && format == ImageFormat::Webp {
            url.push_str("?animated=true");
            separator = '&';
        }
        if let Some(size) = options.size {
            let _ = write!(url, "{separator}size={}", size.pixels());
        }
        url
    }

    fn fixed(base: &str, path: fmt::Arguments<'_>, extension: &str) -> String {
        format!("{base}/{path}.{extension}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATIC: &str = "1269e74af4df7417b13759eae50c83dc";
    const ANIMATED: &str = "a_1269e74af4df7417b13759eae50c83dc";

    fn hash(value: &str) -> ImageHash {
        value.parse().unwrap()
    }

    #[test]
    fn formats_and_animation() {
        let cdn = Cdn::default();
        let guild = GuildId::new(1);
        let icon = |value, options| cdn.guild_icon(guild, hash(value), options);
        let base = "https://cdn.discordapp.com/icons/1";
        assert_eq!(
            icon(STATIC, ImageOptions::new()),
            format!("{base}/{STATIC}.webp")
        );
        assert_eq!(
            icon(ANIMATED, ImageOptions::new()),
            format!("{base}/{ANIMATED}.webp?animated=true")
        );
        assert_eq!(
            icon(
                ANIMATED,
                ImageOptions::new().force_static(true).size(ImageSize::S64)
            ),
            format!("{base}/{ANIMATED}.webp?size=64")
        );
        assert_eq!(
            icon(ANIMATED, ImageOptions::new().format(ImageFormat::Gif)),
            format!("{base}/{ANIMATED}.gif")
        );
        assert_eq!(
            icon(STATIC, ImageOptions::new().format(ImageFormat::Jpeg)),
            format!("{base}/{STATIC}.jpg")
        );
        assert_eq!(
            cdn.emoji(
                EmojiId::new(9),
                true,
                ImageOptions::new().size(ImageSize::S4096)
            ),
            "https://cdn.discordapp.com/emojis/9.webp?animated=true&size=4096"
        );
    }

    #[test]
    fn fixed_urls_and_bases() {
        let cdn = Cdn::new("https://cdn.example.com/", "https://media.example.com");
        assert_eq!(
            cdn.default_user_avatar(3),
            "https://cdn.example.com/embed/avatars/3.png"
        );
        assert_eq!(
            cdn.sticker(StickerId::new(5), StickerFormatType::Gif),
            "https://media.example.com/stickers/5.gif"
        );
        assert_eq!(
            cdn.sticker(StickerId::new(5), StickerFormatType::Lottie),
            "https://cdn.example.com/stickers/5.json"
        );
        assert_eq!(
            cdn.sticker(StickerId::new(5), StickerFormatType::Apng),
            "https://cdn.example.com/stickers/5.png"
        );
        assert_eq!(
            cdn.sticker_pack_banner(GenericId::new(8), ImageOptions::new()),
            "https://cdn.example.com/app-assets/710982414301790216/store/8.webp"
        );
        assert_eq!(
            cdn.soundboard_sound(SoundboardSoundId::new(4)),
            "https://cdn.example.com/soundboard-sounds/4"
        );
        assert_eq!(
            cdn.guild_member_avatar(
                GuildId::new(1),
                UserId::new(2),
                hash(STATIC),
                ImageOptions::new().format(ImageFormat::Png)
            ),
            format!("https://cdn.example.com/guilds/1/users/2/avatars/{STATIC}.png")
        );
    }

    #[test]
    fn every_endpoint_has_the_docs_path() {
        let cdn = Cdn::default();
        let (id, other) = (1, 2);
        let options = ImageOptions::new().format(ImageFormat::Png);
        let hash = hash(STATIC);
        let urls = [
            (cdn.guild_icon(GuildId::new(id), hash, options), "icons/1"),
            (
                cdn.guild_splash(GuildId::new(id), hash, options),
                "splashes/1",
            ),
            (
                cdn.guild_discovery_splash(GuildId::new(id), hash, options),
                "discovery-splashes/1",
            ),
            (
                cdn.guild_banner(GuildId::new(id), hash, options),
                "banners/1",
            ),
            (cdn.user_banner(UserId::new(id), hash, options), "banners/1"),
            (cdn.user_avatar(UserId::new(id), hash, options), "avatars/1"),
            (
                cdn.guild_member_banner(GuildId::new(id), UserId::new(other), hash, options),
                "guilds/1/users/2/banners",
            ),
            (
                cdn.application_icon(ApplicationId::new(id), hash, options),
                "app-icons/1",
            ),
            (
                cdn.application_cover(ApplicationId::new(id), hash, options),
                "app-icons/1",
            ),
            (
                cdn.achievement_icon(ApplicationId::new(id), GenericId::new(other), hash, options),
                "app-assets/1/achievements/2/icons",
            ),
            (
                cdn.team_icon(TeamId::new(id), hash, options),
                "team-icons/1",
            ),
            (
                cdn.role_icon(RoleId::new(id), hash, options),
                "role-icons/1",
            ),
            (
                cdn.guild_scheduled_event_cover(ScheduledEventId::new(id), hash, options),
                "guild-events/1",
            ),
            (
                cdn.guild_tag_badge(GuildId::new(id), hash, options),
                "guild-tag-badges/1",
            ),
            (
                cdn.channel_icon(ChannelId::new(id), hash, options),
                "channel-icons/1",
            ),
        ];
        for (url, path) in urls {
            assert_eq!(
                url,
                format!("https://cdn.discordapp.com/{path}/{STATIC}.png")
            );
        }

        assert_eq!(
            cdn.application_asset(ApplicationId::new(id), "large", options),
            "https://cdn.discordapp.com/app-assets/1/large.png"
        );
        assert_eq!(
            cdn.store_page_asset(ApplicationId::new(id), GenericId::new(other), options),
            "https://cdn.discordapp.com/app-assets/1/store/2.png"
        );
        assert_eq!(
            cdn.avatar_decoration(hash),
            format!("https://cdn.discordapp.com/avatar-decoration-presets/{STATIC}.png")
        );
        assert_eq!(cdn.base(), "https://cdn.discordapp.com");
        assert_eq!(cdn.media_proxy(), "https://media.discordapp.net");
    }

    #[test]
    fn default_avatars_follow_the_discriminator_or_the_id() {
        let user = UserId::new(80_351_110_224_678_912);
        assert_eq!(default_avatar_index(user, 0), 5);

        assert_eq!(default_avatar_index(user, 1337), 2);
        assert_eq!(default_avatar_index(user, 5), 0);
        assert!((1..10_000).all(|discriminator| default_avatar_index(user, discriminator) < 5));
    }

    #[test]
    fn sizes_round_trip() {
        for pixels in [16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
            assert_eq!(ImageSize::from_pixels(pixels).unwrap().pixels(), pixels);
        }
        assert_eq!(ImageSize::from_pixels(0), None);
        assert_eq!(ImageSize::from_pixels(8192), None);
    }
}

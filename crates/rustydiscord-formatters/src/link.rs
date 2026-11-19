use core::fmt::{self, Write as _};

use rustydiscord_model::id::{ApplicationId, ChannelId, GuildId, MessageId, SkuId};

const CHANNELS: &str = "https://discord.com/channels";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Server(Option<GuildId>);

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(guild) => fmt::Display::fmt(&guild, f),
            None => f.write_str("@me"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct ChannelLink {
    channel: ChannelId,
    guild: Server,
}

pub const fn channel_link(channel: ChannelId) -> ChannelLink {
    ChannelLink {
        channel,
        guild: Server(None),
    }
}

impl ChannelLink {
    pub const fn guild(mut self, guild: GuildId) -> Self {
        self.guild = Server(Some(guild));
        self
    }
}

impl fmt::Display for ChannelLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{CHANNELS}/{}/{}", self.guild, self.channel)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct MessageLink {
    channel: ChannelLink,
    message: MessageId,
}

pub const fn message_link(channel: ChannelId, message: MessageId) -> MessageLink {
    MessageLink {
        channel: channel_link(channel),
        message,
    }
}

impl MessageLink {
    pub const fn guild(mut self, guild: GuildId) -> Self {
        self.channel = self.channel.guild(guild);
        self
    }
}

impl fmt::Display for MessageLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.channel, self.message)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct ApplicationDirectory {
    application: ApplicationId,
    sku: Option<SkuId>,
}

pub const fn application_directory(application: ApplicationId) -> ApplicationDirectory {
    ApplicationDirectory {
        application,
        sku: None,
    }
}

impl ApplicationDirectory {
    pub const fn sku(mut self, sku: SkuId) -> Self {
        self.sku = Some(sku);
        self
    }
}

impl fmt::Display for ApplicationDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "https://discord.com/application-directory/{}/store",
            self.application
        )?;
        if let Some(sku) = self.sku {
            write!(f, "/{sku}")?;
        }
        Ok(())
    }
}

fn write_form_encoded(f: &mut fmt::Formatter<'_>, text: &str, lower_case: bool) -> fmt::Result {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for &byte in text.as_bytes() {
        match byte {
            b'A'..=b'Z' if lower_case => f.write_char(char::from(byte.to_ascii_lowercase()))?,
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_' => {
                f.write_char(char::from(byte))?;
            }
            b' ' => f.write_char('+')?,
            _ => {
                f.write_char('%')?;
                f.write_char(char::from(HEX[usize::from(byte >> 4)]))?;
                f.write_char(char::from(HEX[usize::from(byte & 0x0F)]))?;
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct Email<'a> {
    address: &'a str,
    headers: &'a [(&'a str, &'a str)],
}

pub const fn email(address: &str) -> Email<'_> {
    Email {
        address,
        headers: &[],
    }
}

impl<'a> Email<'a> {
    pub const fn headers(mut self, headers: &'a [(&'a str, &'a str)]) -> Self {
        self.headers = headers;
        self
    }
}

impl fmt::Display for Email<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<")?;
        f.write_str(self.address)?;
        for (index, (name, value)) in self.headers.iter().enumerate() {
            f.write_str(if index == 0 { "?" } else { "&" })?;
            write_form_encoded(f, name, true)?;
            f.write_str("=")?;
            write_form_encoded(f, value, false)?;
        }
        f.write_str(">")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct PhoneNumber<'a>(&'a str);

#[must_use]
pub fn phone_number(number: &str) -> Option<PhoneNumber<'_>> {
    number.starts_with('+').then_some(PhoneNumber(number))
}

impl fmt::Display for PhoneNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

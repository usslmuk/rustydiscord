use core::fmt;

use crate::model::id::{ChannelId, CommandId, EmojiId, RoleId, UserId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct UserMention(UserId);

pub const fn user_mention(id: UserId) -> UserMention {
    UserMention(id)
}

impl fmt::Display for UserMention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<@{}>", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct ChannelMention(ChannelId);

pub const fn channel_mention(id: ChannelId) -> ChannelMention {
    ChannelMention(id)
}

impl fmt::Display for ChannelMention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<#{}>", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct RoleMention(RoleId);

pub const fn role_mention(id: RoleId) -> RoleMention {
    RoleMention(id)
}

impl fmt::Display for RoleMention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<@&{}>", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct LinkedRoleMention(RoleId);

pub const fn linked_role_mention(id: RoleId) -> LinkedRoleMention {
    LinkedRoleMention(id)
}

impl fmt::Display for LinkedRoleMention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<id:linked-roles:{}>", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct SlashCommandMention<'a> {
    name: &'a str,
    group: Option<&'a str>,
    subcommand: Option<&'a str>,
    id: CommandId,
}

pub const fn slash_command_mention(name: &str, id: CommandId) -> SlashCommandMention<'_> {
    SlashCommandMention {
        name,
        group: None,
        subcommand: None,
        id,
    }
}

impl<'a> SlashCommandMention<'a> {
    pub const fn subcommand(mut self, subcommand: &'a str) -> Self {
        self.group = None;
        self.subcommand = Some(subcommand);
        self
    }

    pub const fn group(mut self, group: &'a str, subcommand: &'a str) -> Self {
        self.group = Some(group);
        self.subcommand = Some(subcommand);
        self
    }
}

impl fmt::Display for SlashCommandMention<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("</")?;
        f.write_str(self.name)?;
        for part in [self.group, self.subcommand].into_iter().flatten() {
            f.write_str(" ")?;
            f.write_str(part)?;
        }
        write!(f, ":{}>", self.id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct Emoji<'a> {
    id: EmojiId,
    name: Option<&'a str>,
    animated: bool,
}

pub const fn emoji<'a>(id: EmojiId) -> Emoji<'a> {
    Emoji {
        id,
        name: None,
        animated: false,
    }
}

impl<'a> Emoji<'a> {
    pub const fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl fmt::Display for Emoji<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = if self.animated { "<a:" } else { "<:" };
        write!(f, "{prefix}{}:{}>", self.name.unwrap_or("emoji"), self.id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GuildNavigation {
    Browse,

    Customize,

    Guide,

    LinkedRoles,
}

impl GuildNavigation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Browse => "<id:browse>",
            Self::Customize => "<id:customize>",
            Self::Guide => "<id:guide>",
            Self::LinkedRoles => "<id:linked-roles>",
        }
    }
}

impl fmt::Display for GuildNavigation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

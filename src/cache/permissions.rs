use crate::model::{
    channel::Channel,
    id::{GuildId, UserId},
    permissions::Permissions,
    util::Timestamp,
};

use crate::cache::{Backend, Cache, CachedMember};

impl<B: Backend> Cache<B> {
    #[must_use]
    pub fn member_permissions(&self, guild_id: GuildId, user_id: UserId) -> Option<Permissions> {
        let member = self.members(guild_id).get(user_id)?;
        let base = self.guild_level(guild_id, &member)?;
        Some(apply_timeout(base, &member))
    }

    #[must_use]
    pub fn member_permissions_in(&self, channel: &Channel, user_id: UserId) -> Option<Permissions> {
        let guild_id = channel.guild_id()?;
        let member = self.members(guild_id).get(user_id)?;
        let base = self.guild_level(guild_id, &member)?;
        let in_channel = if channel.kind().is_thread() {
            let parent = self.channels().get(channel.parent_id()?)?;
            base.in_channel(
                guild_id,
                user_id,
                &member.roles,
                parent.permission_overwrites(),
            )
        } else {
            base.in_channel(
                guild_id,
                user_id,
                &member.roles,
                channel.permission_overwrites(),
            )
        };
        Some(apply_timeout(in_channel, &member))
    }

    fn guild_level(&self, guild_id: GuildId, member: &CachedMember) -> Option<Permissions> {
        let guild = self.guilds().get(guild_id)?;
        if guild.owner_id == member.user.id {
            return Some(Permissions::all());
        }
        let roles = self.roles(guild_id);

        let everyone = roles.get(guild_id.cast())?.permissions;
        let mut granted = Permissions::empty();
        for &role_id in &*member.roles {
            granted |= roles.get(role_id)?.permissions;
        }
        Some(Permissions::guild_level(false, everyone, [granted]))
    }
}

fn apply_timeout(permissions: Permissions, member: &CachedMember) -> Permissions {
    match member.communication_disabled_until() {
        Some(until) if until > Timestamp::now() => permissions.during_timeout(),
        _ => permissions,
    }
}

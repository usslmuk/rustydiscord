use compact_str::CompactString;

use super::{
    CommandData, CommandDataOption, CommandOptionValue, ComponentData, InteractionChannel,
    InteractionMember, ModalSubmitData, Resolved, SubmittedComponent,
};
use crate::model::{
    guild::Role,
    id::{
        AttachmentId, GenericId, Id, MessageId, RoleId, UserId,
        marker::{ChannelMarker, RoleMarker, UserMarker},
    },
    message::{Attachment, Message},
    user::User,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Mentionable<'a> {
    User {
        user: &'a User,

        member: Option<&'a InteractionMember>,
    },

    Role(&'a Role),
}

impl Mentionable<'_> {
    #[must_use]
    pub const fn id(&self) -> GenericId {
        match self {
            Self::User { user, .. } => user.id.cast(),
            Self::Role(role) => role.id.cast(),
        }
    }
}

impl Resolved {
    fn user(&self, id: UserId) -> Option<&User> {
        self.users.get(&id).map(|user| &**user)
    }

    fn member(&self, id: UserId) -> Option<&InteractionMember> {
        self.members.get(&id).map(|member| &**member)
    }

    fn role(&self, id: RoleId) -> Option<&Role> {
        self.roles.get(&id).map(|role| &**role)
    }

    fn channel(&self, id: Id<ChannelMarker>) -> Option<&InteractionChannel> {
        self.channels.get(&id).map(|channel| &**channel)
    }

    fn message(&self, id: MessageId) -> Option<&Message> {
        self.messages.get(&id).map(|message| &**message)
    }

    fn attachment(&self, id: AttachmentId) -> Option<&Attachment> {
        self.attachments.get(&id).map(|attachment| &**attachment)
    }
}

fn user_and_member(resolved: Option<&Resolved>, id: UserId) -> Option<(&User, &InteractionMember)> {
    let resolved = resolved?;
    Some((resolved.user(id)?, resolved.member(id)?))
}

fn mentionable(resolved: Option<&Resolved>, id: GenericId) -> Option<Mentionable<'_>> {
    let resolved = resolved?;
    if let Some(user) = resolved.user(id.cast()) {
        return Some(Mentionable::User {
            user,
            member: resolved.member(id.cast()),
        });
    }
    resolved.role(id.cast()).map(Mentionable::Role)
}

fn subcommand_option(options: &[CommandDataOption]) -> Option<&CommandDataOption> {
    match options {
        [
            option @ CommandDataOption {
                value: CommandOptionValue::SubCommand(_),
                ..
            },
        ] => Some(option),
        _ => None,
    }
}

impl CommandData {
    #[must_use]
    pub fn leaf_options(&self) -> &[CommandDataOption] {
        let mut options = self.options.as_slice();
        while let [
            CommandDataOption {
                value:
                    CommandOptionValue::SubCommandGroup(inner) | CommandOptionValue::SubCommand(inner),
                ..
            },
        ] = options
        {
            options = inner;
        }
        options
    }

    #[must_use]
    pub fn subcommand(&self) -> Option<&str> {
        let option =
            subcommand_option(&self.options).or_else(|| match self.options.as_slice() {
                [
                    CommandDataOption {
                        value: CommandOptionValue::SubCommandGroup(inner),
                        ..
                    },
                ] => subcommand_option(inner),
                _ => None,
            })?;
        Some(&option.name)
    }

    #[must_use]
    pub fn subcommand_group(&self) -> Option<&str> {
        match self.options.as_slice() {
            [
                CommandDataOption {
                    name,
                    value: CommandOptionValue::SubCommandGroup(_),
                },
            ] => Some(name),
            _ => None,
        }
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&CommandOptionValue> {
        self.leaf_options()
            .iter()
            .find(|option| option.name == name)
            .map(|option| &option.value)
    }

    #[must_use]
    pub fn get_string(&self, name: &str) -> Option<&str> {
        match self.get(name)? {
            CommandOptionValue::String(value) => Some(value),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_integer<T>(&self, name: &str) -> Option<T>
    where
        T: TryFrom<i64>,
    {
        match self.get(name)? {
            CommandOptionValue::Integer(value) => T::try_from(*value).ok(),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_number(&self, name: &str) -> Option<f64> {
        match self.get(name)? {
            CommandOptionValue::Number(value) => Some(*value),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_boolean(&self, name: &str) -> Option<bool> {
        match self.get(name)? {
            CommandOptionValue::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    fn user_id(&self, name: &str) -> Option<UserId> {
        match self.get(name)? {
            CommandOptionValue::User(id) => Some(*id),
            CommandOptionValue::Mentionable(id) => Some(id.cast()),
            _ => None,
        }
    }

    #[must_use]
    pub fn get_user(&self, name: &str) -> Option<&User> {
        self.resolved.as_deref()?.user(self.user_id(name)?)
    }

    #[must_use]
    pub fn get_member(&self, name: &str) -> Option<(&User, &InteractionMember)> {
        user_and_member(self.resolved.as_deref(), self.user_id(name)?)
    }

    #[must_use]
    pub fn get_role(&self, name: &str) -> Option<&Role> {
        let id: RoleId = match self.get(name)? {
            CommandOptionValue::Role(id) => *id,
            CommandOptionValue::Mentionable(id) => id.cast(),
            _ => return None,
        };
        self.resolved.as_deref()?.role(id)
    }

    #[must_use]
    pub fn get_channel(&self, name: &str) -> Option<&InteractionChannel> {
        let CommandOptionValue::Channel(id) = self.get(name)? else {
            return None;
        };
        self.resolved.as_deref()?.channel(*id)
    }

    #[must_use]
    pub fn get_attachment(&self, name: &str) -> Option<&Attachment> {
        let CommandOptionValue::Attachment(id) = self.get(name)? else {
            return None;
        };
        self.resolved.as_deref()?.attachment(*id)
    }

    #[must_use]
    pub fn get_mentionable(&self, name: &str) -> Option<Mentionable<'_>> {
        let CommandOptionValue::Mentionable(id) = self.get(name)? else {
            return None;
        };
        mentionable(self.resolved.as_deref(), *id)
    }

    #[must_use]
    pub fn focused_value(&self) -> Option<&str> {
        match &self.focused()?.value {
            CommandOptionValue::Focused { value, .. } => Some(value),
            _ => None,
        }
    }

    #[must_use]
    pub fn target_user(&self) -> Option<&User> {
        self.resolved.as_deref()?.user(self.target_id?.cast())
    }

    #[must_use]
    pub fn target_member(&self) -> Option<(&User, &InteractionMember)> {
        user_and_member(self.resolved.as_deref(), self.target_id?.cast())
    }

    #[must_use]
    pub fn target_message(&self) -> Option<&Message> {
        self.resolved.as_deref()?.message(self.target_id?.cast())
    }
}

impl ComponentData {
    pub fn users(&self) -> impl Iterator<Item = &User> + '_ {
        let resolved = self.resolved.as_deref();
        self.ids::<UserMarker>()
            .filter_map(move |id| resolved?.user(id))
    }

    pub fn members(&self) -> impl Iterator<Item = (&User, &InteractionMember)> + '_ {
        let resolved = self.resolved.as_deref();
        self.ids::<UserMarker>()
            .filter_map(move |id| user_and_member(resolved, id))
    }

    pub fn roles(&self) -> impl Iterator<Item = &Role> + '_ {
        let resolved = self.resolved.as_deref();
        self.ids::<RoleMarker>()
            .filter_map(move |id| resolved?.role(id))
    }

    pub fn channels(&self) -> impl Iterator<Item = &InteractionChannel> + '_ {
        let resolved = self.resolved.as_deref();
        self.ids::<ChannelMarker>()
            .filter_map(move |id| resolved?.channel(id))
    }

    pub fn mentionables(&self) -> impl Iterator<Item = Mentionable<'_>> + '_ {
        let resolved = self.resolved.as_deref();
        self.ids().filter_map(move |id| mentionable(resolved, id))
    }
}

impl ModalSubmitData {
    #[must_use]
    pub fn string_select_values(&self, custom_id: &str) -> Option<&[CompactString]> {
        match self.get(custom_id)? {
            SubmittedComponent::StringSelect(select) => Some(&select.values),
            _ => None,
        }
    }

    #[must_use]
    pub fn checkbox_group_values(&self, custom_id: &str) -> Option<&[CompactString]> {
        match self.get(custom_id)? {
            SubmittedComponent::CheckboxGroup(group) => Some(&group.values),
            _ => None,
        }
    }

    #[must_use]
    pub fn radio_group_value(&self, custom_id: &str) -> Option<&str> {
        match self.get(custom_id)? {
            SubmittedComponent::RadioGroup(group) => group.value.as_deref(),
            _ => None,
        }
    }

    #[must_use]
    pub fn checkbox_value(&self, custom_id: &str) -> Option<bool> {
        match self.get(custom_id)? {
            SubmittedComponent::Checkbox(checkbox) => Some(checkbox.value),
            _ => None,
        }
    }

    fn user_ids<'a>(&'a self, custom_id: &str) -> Option<impl Iterator<Item = UserId> + use<'a>> {
        let (users, mentionables): (&[UserId], &[GenericId]) = match self.get(custom_id)? {
            SubmittedComponent::UserSelect(select) => (&select.values, &[]),
            SubmittedComponent::MentionableSelect(select) => (&[], &select.values),
            _ => return None,
        };
        Some(
            users
                .iter()
                .copied()
                .chain(mentionables.iter().map(|id| id.cast())),
        )
    }

    #[must_use]
    pub fn selected_users<'a>(
        &'a self,
        custom_id: &str,
    ) -> Option<impl Iterator<Item = &'a User> + use<'a>> {
        let resolved = self.resolved.as_deref();
        Some(
            self.user_ids(custom_id)?
                .filter_map(move |id| resolved?.user(id)),
        )
    }

    #[must_use]
    pub fn selected_members<'a>(
        &'a self,
        custom_id: &str,
    ) -> Option<impl Iterator<Item = (&'a User, &'a InteractionMember)> + use<'a>> {
        let resolved = self.resolved.as_deref();
        Some(
            self.user_ids(custom_id)?
                .filter_map(move |id| user_and_member(resolved, id)),
        )
    }

    #[must_use]
    pub fn selected_roles<'a>(
        &'a self,
        custom_id: &str,
    ) -> Option<impl Iterator<Item = &'a Role> + use<'a>> {
        let (roles, mentionables): (&[RoleId], &[GenericId]) = match self.get(custom_id)? {
            SubmittedComponent::RoleSelect(select) => (&select.values, &[]),
            SubmittedComponent::MentionableSelect(select) => (&[], &select.values),
            _ => return None,
        };
        let resolved = self.resolved.as_deref();
        Some(
            roles
                .iter()
                .copied()
                .chain(mentionables.iter().map(|id| id.cast()))
                .filter_map(move |id| resolved?.role(id)),
        )
    }

    #[must_use]
    pub fn selected_channels<'a>(
        &'a self,
        custom_id: &str,
    ) -> Option<impl Iterator<Item = &'a InteractionChannel> + use<'a>> {
        let SubmittedComponent::ChannelSelect(select) = self.get(custom_id)? else {
            return None;
        };
        let resolved = self.resolved.as_deref();
        Some(
            select
                .values
                .iter()
                .filter_map(move |id: &Id<ChannelMarker>| resolved?.channel(*id)),
        )
    }

    #[must_use]
    pub fn selected_mentionables<'a>(
        &'a self,
        custom_id: &str,
    ) -> Option<impl Iterator<Item = Mentionable<'a>> + use<'a>> {
        let SubmittedComponent::MentionableSelect(select) = self.get(custom_id)? else {
            return None;
        };
        let resolved = self.resolved.as_deref();
        Some(
            select
                .values
                .iter()
                .filter_map(move |id| mentionable(resolved, *id)),
        )
    }

    #[must_use]
    pub fn uploaded_files<'a>(
        &'a self,
        custom_id: &str,
    ) -> Option<impl Iterator<Item = &'a Attachment> + use<'a>> {
        let SubmittedComponent::FileUpload(upload) = self.get(custom_id)? else {
            return None;
        };
        let resolved = self.resolved.as_deref();
        Some(
            upload
                .values
                .iter()
                .filter_map(move |id| resolved?.attachment(*id)),
        )
    }
}

use std::{future::Future, sync::Arc, time::Duration};

use crate::cache::{Backend, CachedGuild, CachedMember, CachedMessage, CachedUser};
use crate::collection::Collection;
use crate::model::{
    audit_log::AuditLog,
    channel::Channel,
    gateway::payload::{GuildMemberAdd, GuildMemberUpdate, MessageCreate},
    guild::{Guild, Integration},
    id::{
        ApplicationId, ChannelId, GuildId, IntegrationId, InteractionId, MessageId, RoleId, UserId,
        WebhookId,
    },
    interaction::{
        AutocompleteCallbackData, AutocompleteChoices, DeferredCallbackData, Interaction,
        InteractionResponse, MessageCallbackData, ModalCallbackData,
    },
    invite::InviteWithMetadata,
    message::{Message, MessageFlags, MessageReference},
    permissions::Permissions,
    rest::{
        CreateChannelInvite, CreateFollowupMessage, CreateGuildBan, CreateMessage, CreateWebhook,
        EditMessage, EditWebhookMessage, ExecuteWebhook, GetGuildAuditLog, ModifyGuild,
        ModifyGuildChannel, ModifyGuildMember, ModifyWebhook, SendSoundboardSound,
        StartThreadFromMessage,
    },
    user::User,
    util::Timestamp,
    webhook::Webhook,
};
use crate::rest::{Error, ReactionEmoji};

use crate::{
    Context,
    collector::{CollectorBuilder, Key, ReactionEvent, mode},
    manager::{
        AutoModerationRuleManager, BanManager, EmojiManager, GuildChannelManager,
        GuildCommandManager, MemberManager, MessageManager, OverwriteTarget, RoleManager,
        ScheduledEventManager, SoundboardSoundManager, StageInstanceManager, StickerManager,
        ThreadManager, ThreadMemberManager,
    },
    router::{ComponentInteraction, ModalInteraction},
    webhook::WebhookError,
};

pub trait MessageExt: sealed::Message {
    fn reply<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<CreateMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (channel_id, message_id) = self.ids();
        let mut body = body.into();
        body.message_reference = Some(MessageReference::reply(message_id));
        ctx.messages(channel_id).send(body)
    }

    fn edit<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &EditMessage,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).edit(message_id, body)
    }

    fn delete<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).delete(message_id)
    }

    fn react<'e, B: Backend>(
        &self,
        ctx: &Context<B>,
        emoji: impl Into<ReactionEmoji<'e>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).react(message_id, emoji)
    }

    fn pin<B: Backend>(
        &self,
        ctx: &Context<B>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).pin(message_id, reason)
    }

    fn unpin<B: Backend>(
        &self,
        ctx: &Context<B>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).unpin(message_id, reason)
    }

    fn start_thread<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &StartThreadFromMessage,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Arc<Channel>, Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id)
            .start_thread(message_id, body, reason)
    }

    fn crosspost<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).crosspost(message_id)
    }

    fn remove_reaction<'e, B: Backend>(
        &self,
        ctx: &Context<B>,
        emoji: impl Into<ReactionEmoji<'e>>,
        user: Option<UserId>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id)
            .remove_reaction(message_id, emoji, user)
    }

    fn remove_all_reactions<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (channel_id, message_id) = self.ids();
        ctx.messages(channel_id).remove_all_reactions(message_id)
    }

    fn reaction_collector<B: Backend>(&self, ctx: &Context<B>) -> CollectorBuilder<ReactionEvent> {
        let (channel_id, message_id) = self.ids();
        CollectorBuilder::new(
            ctx,
            Key::Message(message_id),
            self.guild(),
            Some(channel_id),
        )
    }

    fn await_reactions<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ReactionEvent, mode::All> {
        let (channel_id, message_id) = self.ids();
        CollectorBuilder::new(
            ctx,
            Key::Message(message_id),
            self.guild(),
            Some(channel_id),
        )
    }

    fn component_collector<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ComponentInteraction> {
        let (channel_id, message_id) = self.ids();
        CollectorBuilder::new(
            ctx,
            Key::Message(message_id),
            self.guild(),
            Some(channel_id),
        )
    }

    fn await_component<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ComponentInteraction, mode::First> {
        let (channel_id, message_id) = self.ids();
        CollectorBuilder::new(
            ctx,
            Key::Message(message_id),
            self.guild(),
            Some(channel_id),
        )
    }
}

impl MessageExt for Message {}
impl MessageExt for CachedMessage {}

pub trait ChannelExt: sealed::Channel {
    fn messages<'a, B: Backend>(&self, ctx: &'a Context<B>) -> MessageManager<'a, B> {
        ctx.messages(self.channel_id())
    }

    fn send<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<CreateMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        ctx.messages(self.channel_id()).send(body)
    }

    fn send_typing<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.channels().send_typing(self.channel_id())
    }

    fn bulk_delete<B: Backend>(
        &self,
        ctx: &Context<B>,
        message_ids: Vec<MessageId>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.messages(self.channel_id())
            .bulk_delete(message_ids, reason)
    }

    fn edit<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &ModifyGuildChannel,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Arc<Channel>, Error>> + Send {
        ctx.channels().edit(self.channel_id(), body, reason)
    }

    fn delete<B: Backend>(
        &self,
        ctx: &Context<B>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send {
        ctx.channels().delete(self.channel_id(), reason)
    }

    fn set_permissions<B: Backend>(
        &self,
        ctx: &Context<B>,
        target: impl Into<OverwriteTarget>,
        allow: Permissions,
        deny: Permissions,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.channels()
            .set_permissions(self.channel_id(), target, allow, deny, reason)
    }

    fn delete_permissions<B: Backend>(
        &self,
        ctx: &Context<B>,
        target: impl Into<OverwriteTarget>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.channels()
            .delete_permissions(self.channel_id(), target, reason)
    }

    fn create_webhook<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &CreateWebhook,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send {
        ctx.webhooks().create(self.channel_id(), body, reason)
    }

    fn webhooks<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Collection<WebhookId, Webhook>, Error>> + Send {
        ctx.webhooks().list_channel(self.channel_id())
    }

    fn create_invite<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &CreateChannelInvite,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<InviteWithMetadata, Error>> + Send {
        ctx.invites().create(self.channel_id(), body, reason)
    }

    fn invites<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Collection<String, InviteWithMetadata>, Error>> + Send {
        ctx.invites().list_channel(self.channel_id())
    }

    fn threads<'a, B: Backend>(&self, ctx: &'a Context<B>) -> ThreadManager<'a, B> {
        ctx.threads(self.channel_id())
    }

    fn thread_members<'a, B: Backend>(&self, ctx: &'a Context<B>) -> ThreadMemberManager<'a, B> {
        ctx.thread_members(self.channel_id())
    }

    fn send_soundboard_sound<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &SendSoundboardSound,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.channels()
            .send_soundboard_sound(self.channel_id(), body)
    }

    fn message_collector<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<Arc<MessageCreate>> {
        ctx.message_collector(self.channel_id())
    }

    fn await_messages<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<Arc<MessageCreate>, mode::All> {
        ctx.await_messages(self.channel_id())
    }

    fn component_collector<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ComponentInteraction> {
        ctx.component_collector(self.channel_id())
    }

    fn await_component<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ComponentInteraction, mode::First> {
        ctx.await_component(self.channel_id())
    }
}

impl ChannelExt for Channel {}

pub trait UserExt: sealed::User {
    fn send<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<CreateMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        ctx.users().send(self.user_id(), body)
    }

    fn create_dm<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Arc<Channel>, Error>> + Send {
        ctx.users().create_dm(self.user_id())
    }
}

impl UserExt for User {}
impl UserExt for CachedUser {}

pub trait MemberExt: sealed::Member {
    fn edit<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &ModifyGuildMember,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Arc<CachedMember>, Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id).edit(user_id, body, reason)
    }

    fn timeout<B: Backend>(
        &self,
        ctx: &Context<B>,
        until: Option<Timestamp>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Arc<CachedMember>, Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id).timeout(user_id, until, reason)
    }

    fn set_nickname<B: Backend>(
        &self,
        ctx: &Context<B>,
        nickname: Option<&str>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Arc<CachedMember>, Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id)
            .set_nickname(user_id, nickname, reason)
    }

    fn add_role<B: Backend>(
        &self,
        ctx: &Context<B>,
        role_id: RoleId,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id).add_role(user_id, role_id, reason)
    }

    fn remove_role<B: Backend>(
        &self,
        ctx: &Context<B>,
        role_id: RoleId,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id).remove_role(user_id, role_id, reason)
    }

    fn kick<B: Backend>(
        &self,
        ctx: &Context<B>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id).kick(user_id, reason)
    }

    fn ban<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &CreateGuildBan,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (guild_id, user_id) = self.ids();
        ctx.members(guild_id).ban(user_id, body, reason)
    }

    fn permissions<B: Backend>(&self, ctx: &Context<B>) -> Option<Permissions> {
        let (guild_id, user_id) = self.ids();
        ctx.ops().member_permissions(guild_id, user_id)
    }

    fn permissions_in<B: Backend>(
        &self,
        ctx: &Context<B>,
        channel_id: ChannelId,
    ) -> Option<Permissions> {
        let (guild_id, user_id) = self.ids();
        ctx.ops()
            .member_permissions_in(guild_id, channel_id, user_id)
    }
}

impl MemberExt for CachedMember {}
impl MemberExt for GuildMemberAdd {}
impl MemberExt for GuildMemberUpdate {}

pub trait GuildExt: sealed::Guild {
    fn members<'a, B: Backend>(&self, ctx: &'a Context<B>) -> MemberManager<'a, B> {
        ctx.members(self.guild_id())
    }

    fn roles<'a, B: Backend>(&self, ctx: &'a Context<B>) -> RoleManager<'a, B> {
        ctx.roles(self.guild_id())
    }

    fn bans<'a, B: Backend>(&self, ctx: &'a Context<B>) -> BanManager<'a, B> {
        ctx.bans(self.guild_id())
    }

    fn channels<'a, B: Backend>(&self, ctx: &'a Context<B>) -> GuildChannelManager<'a, B> {
        ctx.guild_channels(self.guild_id())
    }

    fn edit<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &ModifyGuild,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Arc<CachedGuild>, Error>> + Send {
        ctx.guilds().edit(self.guild_id(), body, reason)
    }

    fn leave<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.guilds().leave(self.guild_id())
    }

    fn emojis<'a, B: Backend>(&self, ctx: &'a Context<B>) -> EmojiManager<'a, B> {
        ctx.emojis(self.guild_id())
    }

    fn stickers<'a, B: Backend>(&self, ctx: &'a Context<B>) -> StickerManager<'a, B> {
        ctx.stickers(self.guild_id())
    }

    fn webhooks<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Collection<WebhookId, Webhook>, Error>> + Send {
        ctx.webhooks().list_guild(self.guild_id())
    }

    fn invites<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Collection<String, InviteWithMetadata>, Error>> + Send {
        ctx.invites().list_guild(self.guild_id())
    }

    fn scheduled_events<'a, B: Backend>(
        &self,
        ctx: &'a Context<B>,
    ) -> ScheduledEventManager<'a, B> {
        ctx.scheduled_events(self.guild_id())
    }

    fn stage_instances<'a, B: Backend>(&self, ctx: &'a Context<B>) -> StageInstanceManager<'a, B> {
        ctx.stage_instances(self.guild_id())
    }

    fn soundboard_sounds<'a, B: Backend>(
        &self,
        ctx: &'a Context<B>,
    ) -> SoundboardSoundManager<'a, B> {
        ctx.soundboard_sounds(self.guild_id())
    }

    #[track_caller]
    fn commands<'a, B: Backend>(&self, ctx: &'a Context<B>) -> GuildCommandManager<'a, B> {
        ctx.guild_commands(self.guild_id())
    }

    fn auto_moderation_rules<'a, B: Backend>(
        &self,
        ctx: &'a Context<B>,
    ) -> AutoModerationRuleManager<'a, B> {
        ctx.auto_moderation_rules(self.guild_id())
    }

    fn audit_log<B: Backend>(
        &self,
        ctx: &Context<B>,
        query: &GetGuildAuditLog,
    ) -> impl Future<Output = Result<AuditLog, Error>> + Send {
        ctx.guilds().audit_log(self.guild_id(), query)
    }

    fn integrations<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Collection<IntegrationId, Integration>, Error>> + Send {
        ctx.guilds().integrations(self.guild_id())
    }
}

impl GuildExt for Guild {}
impl GuildExt for CachedGuild {}

pub trait WebhookExt: sealed::Webhook {
    fn send<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<ExecuteWebhook>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, WebhookError>> + Send {
        let (webhook_id, token) = self.webhook_parts();
        let body = body.into();
        let webhooks = ctx.webhooks();
        async move {
            let token = token.ok_or(WebhookError::MissingToken)?;
            Ok(webhooks.send(webhook_id, token, body).await?)
        }
    }

    fn edit<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: &ModifyWebhook,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send {
        ctx.webhooks().edit(self.webhook_parts().0, body, reason)
    }

    fn delete<B: Backend>(
        &self,
        ctx: &Context<B>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        ctx.webhooks().delete(self.webhook_parts().0, reason)
    }
}

impl WebhookExt for Webhook {}

pub trait InteractionExt: sealed::Interaction {
    fn reply<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<MessageCallbackData>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let response = InteractionResponse::ChannelMessageWithSource(Box::new(body.into()));
        callback(ctx, self.interaction_parts(), response)
    }

    fn defer_reply<B: Backend>(
        &self,
        ctx: &Context<B>,
        flags: MessageFlags,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let response =
            InteractionResponse::DeferredChannelMessageWithSource(DeferredCallbackData::new(flags));
        callback(ctx, self.interaction_parts(), response)
    }

    fn update<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<MessageCallbackData>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let response = InteractionResponse::UpdateMessage(Box::new(body.into()));
        callback(ctx, self.interaction_parts(), response)
    }

    fn defer_update<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        callback(
            ctx,
            self.interaction_parts(),
            InteractionResponse::DeferredUpdateMessage,
        )
    }

    fn show_modal<B: Backend>(
        &self,
        ctx: &Context<B>,
        modal: ModalCallbackData,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let response = InteractionResponse::Modal(Box::new(modal));
        callback(ctx, self.interaction_parts(), response)
    }

    fn respond<B: Backend>(
        &self,
        ctx: &Context<B>,
        choices: AutocompleteChoices,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let response =
            InteractionResponse::AutocompleteResult(AutocompleteCallbackData::new(choices));
        callback(ctx, self.interaction_parts(), response)
    }

    fn launch_activity<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        callback(
            ctx,
            self.interaction_parts(),
            InteractionResponse::LaunchActivity,
        )
    }

    fn fetch_reply<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        async move {
            let message = ctx
                .rest()
                .get_original_interaction_response(application_id, token)
                .await?;
            Ok(ctx.store_message(&message))
        }
    }

    fn edit_reply<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<EditWebhookMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        let body = body.into();
        async move {
            let message = ctx
                .rest()
                .edit_original_interaction_response(application_id, token, &body)
                .await?;
            Ok(ctx.store_message(&message))
        }
    }

    fn delete_reply<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        async move {
            ctx.rest()
                .delete_original_interaction_response(application_id, token)
                .await
        }
    }

    fn follow_up<B: Backend>(
        &self,
        ctx: &Context<B>,
        body: impl Into<CreateFollowupMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        let body = body.into();
        async move {
            let message = ctx
                .rest()
                .create_followup_message(application_id, token, &body)
                .await?;
            Ok(ctx.store_message(&message))
        }
    }

    fn fetch_follow_up<B: Backend>(
        &self,
        ctx: &Context<B>,
        message_id: MessageId,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        async move {
            let message = ctx
                .rest()
                .get_followup_message(application_id, token, message_id)
                .await?;
            Ok(ctx.store_message(&message))
        }
    }

    fn edit_follow_up<B: Backend>(
        &self,
        ctx: &Context<B>,
        message_id: MessageId,
        body: impl Into<EditWebhookMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        let body = body.into();
        async move {
            let message = ctx
                .rest()
                .edit_followup_message(application_id, token, message_id, &body)
                .await?;
            Ok(ctx.store_message(&message))
        }
    }

    fn delete_follow_up<B: Backend>(
        &self,
        ctx: &Context<B>,
        message_id: MessageId,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (_, application_id, token) = self.interaction_parts();
        async move {
            ctx.rest()
                .delete_followup_message(application_id, token, message_id)
                .await
        }
    }

    fn component_collector<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ComponentInteraction> {
        let interaction = self.interaction();
        CollectorBuilder::new(
            ctx,
            reply_key(interaction),
            interaction.guild_id,
            interaction.channel_id,
        )
    }

    fn await_component<B: Backend>(
        &self,
        ctx: &Context<B>,
    ) -> CollectorBuilder<ComponentInteraction, mode::First> {
        let interaction = self.interaction();
        CollectorBuilder::new(
            ctx,
            reply_key(interaction),
            interaction.guild_id,
            interaction.channel_id,
        )
    }

    fn await_modal_submit<B: Backend>(
        &self,
        ctx: &Context<B>,
        time: Duration,
    ) -> CollectorBuilder<ModalInteraction, mode::First> {
        let interaction = self.interaction();

        let user = interaction
            .author()
            .map_or(UserId::new(u64::MAX), |user| user.id);
        CollectorBuilder::new(ctx, Key::User(user), interaction.guild_id, None).time(time)
    }
}

impl InteractionExt for Interaction {}
impl InteractionExt for crate::router::CommandInteraction {}
impl InteractionExt for crate::router::AutocompleteInteraction {}
impl InteractionExt for crate::router::ComponentInteraction {}
impl InteractionExt for crate::router::ModalInteraction {}

fn reply_key(interaction: &Interaction) -> Key {
    match &interaction.message {
        Some(message) => Key::Message(message.id),
        None => Key::Interaction(interaction.id),
    }
}

async fn callback<B: Backend>(
    ctx: &Context<B>,
    (interaction_id, _, token): (InteractionId, ApplicationId, &str),
    response: InteractionResponse,
) -> Result<(), Error> {
    #[cfg(feature = "http-interactions")]
    if let Some(initial) = ctx.core().take_inline(interaction_id) {
        return crate::http_interactions::respond(
            ctx.rest(),
            initial,
            interaction_id,
            token,
            &response,
        )
        .await;
    }
    ctx.rest()
        .create_interaction_response(interaction_id, token, &response)
        .await
}

mod sealed {
    use super::{ApplicationId, ChannelId, GuildId, InteractionId, MessageId, UserId, WebhookId};

    pub trait Interaction {
        fn interaction_parts(&self) -> (InteractionId, ApplicationId, &str);

        fn interaction(&self) -> &crate::model::interaction::Interaction;
    }

    impl Interaction for crate::model::interaction::Interaction {
        fn interaction_parts(&self) -> (InteractionId, ApplicationId, &str) {
            (self.id, self.application_id, &self.token)
        }

        fn interaction(&self) -> &Self {
            self
        }
    }

    macro_rules! wrapped {
        ($($wrapper:ident),*) => {$(
            impl Interaction for crate::router::$wrapper {
                fn interaction_parts(&self) -> (InteractionId, ApplicationId, &str) {
                    crate::router::$wrapper::interaction(self).interaction_parts()
                }

                fn interaction(&self) -> &crate::model::interaction::Interaction {
                    crate::router::$wrapper::interaction(self)
                }
            }
        )*};
    }

    wrapped!(
        CommandInteraction,
        AutocompleteInteraction,
        ComponentInteraction,
        ModalInteraction
    );

    pub trait Message {
        fn ids(&self) -> (ChannelId, MessageId);

        fn guild(&self) -> Option<GuildId>;
    }

    impl Message for crate::model::message::Message {
        fn ids(&self) -> (ChannelId, MessageId) {
            (self.channel_id, self.id)
        }

        fn guild(&self) -> Option<GuildId> {
            self.guild_id
        }
    }

    impl Message for crate::cache::CachedMessage {
        fn ids(&self) -> (ChannelId, MessageId) {
            (self.channel_id, self.id)
        }

        fn guild(&self) -> Option<GuildId> {
            self.guild_id
        }
    }

    pub trait Channel {
        fn channel_id(&self) -> ChannelId;
    }

    impl Channel for crate::model::channel::Channel {
        fn channel_id(&self) -> ChannelId {
            self.id()
        }
    }

    pub trait User {
        fn user_id(&self) -> UserId;
    }

    impl User for crate::model::user::User {
        fn user_id(&self) -> UserId {
            self.id
        }
    }

    impl User for crate::cache::CachedUser {
        fn user_id(&self) -> UserId {
            self.id
        }
    }

    pub trait Member {
        fn ids(&self) -> (GuildId, UserId);
    }

    impl Member for crate::cache::CachedMember {
        fn ids(&self) -> (GuildId, UserId) {
            (self.guild_id, self.user.id)
        }
    }

    impl Member for crate::model::gateway::payload::GuildMemberAdd {
        fn ids(&self) -> (GuildId, UserId) {
            (self.guild_id, self.member.user.id)
        }
    }

    impl Member for crate::model::gateway::payload::GuildMemberUpdate {
        fn ids(&self) -> (GuildId, UserId) {
            (self.guild_id, self.user.id)
        }
    }

    pub trait Guild {
        fn guild_id(&self) -> GuildId;
    }

    pub trait Webhook {
        fn webhook_parts(&self) -> (WebhookId, Option<&str>);
    }

    impl Webhook for crate::model::webhook::Webhook {
        fn webhook_parts(&self) -> (WebhookId, Option<&str>) {
            (self.id, self.token.as_deref())
        }
    }

    impl Guild for crate::model::guild::Guild {
        fn guild_id(&self) -> GuildId {
            self.id
        }
    }

    impl Guild for crate::cache::CachedGuild {
        fn guild_id(&self) -> GuildId {
            self.id
        }
    }
}

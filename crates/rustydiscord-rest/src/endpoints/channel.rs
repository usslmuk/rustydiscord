use rustydiscord_model::{
    channel::{Channel, FollowedChannel, ThreadMember},
    id::{ChannelId, GenericId, MessageId, RoleId, UserId},
    invite::InviteWithMetadata,
    permissions::Permissions,
    rest::{
        ArchivedThreads, CreateChannelInvite, EditChannelPermissions, FollowAnnouncementChannel,
        ForumThread, GetChannelPermissions, GetThreadMember, GroupDmAddRecipient,
        ListArchivedThreads, ListJoinedPrivateArchivedThreads, ListThreadMembers, ModifyGroupDm,
        ModifyGuildChannel, ModifyThread, SetVoiceChannelStatus, StartThreadFromMessage,
        StartThreadInForumOrMediaChannel, StartThreadWithoutMessage,
    },
};

use super::PermissionsResponse;
use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_channel(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetChannel { channel_id }))
                .await
        })
    }

    pub fn modify_channel<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a ModifyGuildChannel,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyChannel { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_thread<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a ModifyThread,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyChannel { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_group_dm<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a ModifyGroupDm,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ModifyChannel { channel_id }).json(body)?)
                .await
        })
    }

    pub fn delete_channel<'a>(
        &'a self,
        channel_id: ChannelId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteChannel { channel_id }).reason(reason))
                .await
        })
    }

    pub fn get_channel_member_permissions<'a>(
        &'a self,
        channel_id: ChannelId,
        user_id: UserId,
        query: &'a GetChannelPermissions,
    ) -> impl Future<Output = Result<Permissions, Error>> + Send + 'a {
        boxed(async move {
            let response: PermissionsResponse = self
                .request(
                    Request::new(Route::GetChannelMemberPermissions {
                        channel_id,
                        user_id,
                    })
                    .query(query),
                )
                .await?;
            Ok(response.permissions)
        })
    }

    pub fn get_channel_role_permissions<'a>(
        &'a self,
        channel_id: ChannelId,
        role_id: RoleId,
        query: &'a GetChannelPermissions,
    ) -> impl Future<Output = Result<Permissions, Error>> + Send + 'a {
        boxed(async move {
            let response: PermissionsResponse = self
                .request(
                    Request::new(Route::GetChannelRolePermissions {
                        channel_id,
                        role_id,
                    })
                    .query(query),
                )
                .await?;
            Ok(response.permissions)
        })
    }

    pub fn set_voice_channel_status<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a SetVoiceChannelStatus,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::SetVoiceChannelStatus { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn edit_channel_permissions<'a>(
        &'a self,
        channel_id: ChannelId,
        overwrite_id: GenericId,
        body: &'a EditChannelPermissions,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::EditChannelPermissions {
                    channel_id,
                    overwrite_id,
                })
                .json(body)?
                .reason(reason),
            )
            .await
        })
    }

    pub fn delete_channel_permission<'a>(
        &'a self,
        channel_id: ChannelId,
        overwrite_id: GenericId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::DeleteChannelPermission {
                    channel_id,
                    overwrite_id,
                })
                .reason(reason),
            )
            .await
        })
    }

    pub fn get_channel_invites(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<Vec<InviteWithMetadata>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetChannelInvites { channel_id }))
                .await
        })
    }

    pub fn create_channel_invite<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a CreateChannelInvite,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<InviteWithMetadata, Error>> + Send + 'a {
        boxed(async move {
            let mut request = Request::new(Route::CreateChannelInvite { channel_id })
                .json(body)?
                .reason(reason);
            if let Some(file) = &body.target_users_file {
                request = request.form_file("target_users_file", file);
            }
            self.call(request).await
        })
    }

    pub fn follow_announcement_channel<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a FollowAnnouncementChannel,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<FollowedChannel, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::FollowAnnouncementChannel { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn trigger_typing_indicator(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(Route::TriggerTypingIndicator { channel_id }))
                .await
        })
    }

    pub fn group_dm_add_recipient<'a>(
        &'a self,
        channel_id: ChannelId,
        user_id: UserId,
        body: &'a GroupDmAddRecipient,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::GroupDmAddRecipient {
                    channel_id,
                    user_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn group_dm_remove_recipient(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(Route::GroupDmRemoveRecipient {
                channel_id,
                user_id,
            }))
            .await
        })
    }

    pub fn start_thread_from_message<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        body: &'a StartThreadFromMessage,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::StartThreadFromMessage {
                    channel_id,
                    message_id,
                })
                .json(body)?
                .reason(reason),
            )
            .await
        })
    }

    pub fn start_thread_without_message<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a StartThreadWithoutMessage,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::StartThreadWithoutMessage { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn start_thread_in_forum_or_media_channel<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a StartThreadInForumOrMediaChannel,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<ForumThread, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::StartThreadInForumOrMediaChannel { channel_id })
                    .json(body)?
                    .files(&body.message.files)
                    .reason(reason),
            )
            .await
        })
    }

    pub fn join_thread(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::JoinThread { channel_id }))
                .await
        })
    }

    pub fn add_thread_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::AddThreadMember {
                channel_id,
                user_id,
            }))
            .await
        })
    }

    pub fn leave_thread(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::LeaveThread { channel_id }))
                .await
        })
    }

    pub fn remove_thread_member(
        &self,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::RemoveThreadMember {
                channel_id,
                user_id,
            }))
            .await
        })
    }

    pub fn get_thread_member<'a>(
        &'a self,
        channel_id: ChannelId,
        user_id: UserId,
        query: &'a GetThreadMember,
    ) -> impl Future<Output = Result<ThreadMember, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::GetThreadMember {
                    channel_id,
                    user_id,
                })
                .query(query),
            )
            .await
        })
    }

    pub fn list_thread_members<'a>(
        &'a self,
        channel_id: ChannelId,
        query: &'a ListThreadMembers,
    ) -> impl Future<Output = Result<Vec<ThreadMember>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListThreadMembers { channel_id }).query(query))
                .await
        })
    }

    pub fn list_public_archived_threads<'a>(
        &'a self,
        channel_id: ChannelId,
        query: &'a ListArchivedThreads,
    ) -> impl Future<Output = Result<ArchivedThreads, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListPublicArchivedThreads { channel_id }).query(query))
                .await
        })
    }

    pub fn list_private_archived_threads<'a>(
        &'a self,
        channel_id: ChannelId,
        query: &'a ListArchivedThreads,
    ) -> impl Future<Output = Result<ArchivedThreads, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListPrivateArchivedThreads { channel_id }).query(query))
                .await
        })
    }

    pub fn list_joined_private_archived_threads<'a>(
        &'a self,
        channel_id: ChannelId,
        query: &'a ListJoinedPrivateArchivedThreads,
    ) -> impl Future<Output = Result<ArchivedThreads, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ListJoinedPrivateArchivedThreads { channel_id }).query(query),
            )
            .await
        })
    }
}

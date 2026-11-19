use rustydiscord_model::{
    id::{ChannelId, LobbyId, MessageId, UserId},
    lobby::{Lobby, LobbyInvite, LobbyMember, LobbyMessage},
    rest::{
        AddLobbyMember, BulkLobbyMember, CreateLobby, CreateOrJoinLobby, GetLobbyMessages,
        LobbyMetadata, ModifyLobby, SendLobbyMessage,
    },
};
use serde::Serialize;

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

#[derive(Serialize)]
struct ChannelLinking {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<ChannelId>,
}

impl Rest {
    pub fn create_lobby<'a>(
        &'a self,
        body: &'a CreateLobby,
    ) -> impl Future<Output = Result<Lobby, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateLobby).json(body)?)
                .await
        })
    }

    pub fn create_or_join_lobby<'a>(
        &'a self,
        body: &'a CreateOrJoinLobby,
    ) -> impl Future<Output = Result<Lobby, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateOrJoinLobby).json(body)?)
                .await
        })
    }

    pub fn get_lobby(
        &self,
        lobby_id: LobbyId,
    ) -> impl Future<Output = Result<Lobby, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::GetLobby { lobby_id })).await })
    }

    pub fn modify_lobby<'a>(
        &'a self,
        lobby_id: LobbyId,
        body: &'a ModifyLobby,
    ) -> impl Future<Output = Result<Lobby, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ModifyLobby { lobby_id }).json(body)?)
                .await
        })
    }

    pub fn delete_lobby(
        &self,
        lobby_id: LobbyId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(Route::DeleteLobby { lobby_id }))
                .await
        })
    }
}

impl Rest {
    pub fn add_lobby_member<'a>(
        &'a self,
        lobby_id: LobbyId,
        user_id: UserId,
        body: &'a AddLobbyMember,
    ) -> impl Future<Output = Result<LobbyMember, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::AddLobbyMember { lobby_id, user_id }).json(body)?)
                .await
        })
    }

    pub fn bulk_update_lobby_members<'a>(
        &'a self,
        lobby_id: LobbyId,
        members: &'a [BulkLobbyMember],
    ) -> impl Future<Output = Result<Vec<LobbyMember>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::BulkUpdateLobbyMembers { lobby_id }).json(members)?)
                .await
        })
    }

    pub fn remove_lobby_member(
        &self,
        lobby_id: LobbyId,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(Route::RemoveLobbyMember { lobby_id, user_id }))
                .await
        })
    }

    pub fn leave_lobby(
        &self,
        lobby_id: LobbyId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(Route::LeaveLobby { lobby_id }))
                .await
        })
    }
}

impl Rest {
    pub fn link_channel_to_lobby(
        &self,
        lobby_id: LobbyId,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<Lobby, Error>> + Send + '_ {
        boxed(async move {
            let body = ChannelLinking {
                channel_id: Some(channel_id),
            };
            self.call(Request::new(Route::LinkChannelToLobby { lobby_id }).json(&body)?)
                .await
        })
    }

    pub fn unlink_channel_from_lobby(
        &self,
        lobby_id: LobbyId,
    ) -> impl Future<Output = Result<Lobby, Error>> + Send + '_ {
        boxed(async move {
            let body = ChannelLinking { channel_id: None };
            self.call(Request::new(Route::UnlinkChannelFromLobby { lobby_id }).json(&body)?)
                .await
        })
    }

    pub fn create_lobby_channel_invite_for_self(
        &self,
        lobby_id: LobbyId,
    ) -> impl Future<Output = Result<LobbyInvite, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::CreateLobbyChannelInviteForSelf {
                lobby_id,
            }))
            .await
        })
    }

    pub fn create_lobby_channel_invite_for_user(
        &self,
        lobby_id: LobbyId,
        user_id: UserId,
    ) -> impl Future<Output = Result<LobbyInvite, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::CreateLobbyChannelInviteForUser {
                lobby_id,
                user_id,
            }))
            .await
        })
    }
}

impl Rest {
    pub fn send_lobby_message<'a>(
        &'a self,
        lobby_id: LobbyId,
        body: &'a SendLobbyMessage,
    ) -> impl Future<Output = Result<LobbyMessage, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::SendLobbyMessage { lobby_id }).json(body)?)
                .await
        })
    }

    pub fn get_lobby_messages<'a>(
        &'a self,
        lobby_id: LobbyId,
        query: &'a GetLobbyMessages,
    ) -> impl Future<Output = Result<Vec<LobbyMessage>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetLobbyMessages { lobby_id }).query(query))
                .await
        })
    }

    pub fn update_lobby_message_moderation_metadata<'a>(
        &'a self,
        lobby_id: LobbyId,
        message_id: MessageId,
        metadata: &'a LobbyMetadata,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::UpdateLobbyMessageModerationMetadata {
                    lobby_id,
                    message_id,
                })
                .json(metadata)?,
            )
            .await
        })
    }
}

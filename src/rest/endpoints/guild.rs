use std::collections::HashMap;

use crate::model::{
    channel::Channel,
    guild::{
        Ban, Guild, IncidentsData, Integration, Member, MfaLevel, Onboarding, Preview, Role,
        WelcomeScreen, Widget, WidgetSettings,
    },
    id::{GuildId, IntegrationId, RoleId, UserId},
    invite::InviteWithMetadata,
    permissions::Permissions,
    rest::{
        ActiveThreads, AddGuildMember, BeginGuildPrune, BulkBanResponse, BulkGuildBan, CreateGuild,
        CreateGuildBan, CreateGuildChannel, CreateGuildRole, GetGuild, GetGuildBans,
        GetGuildPruneCount, GuildMembershipScreening, ListGuildMembers, ModifyCurrentMember,
        ModifyCurrentMemberNickname, ModifyGuild, ModifyGuildChannelPosition,
        ModifyGuildIncidentActions, ModifyGuildMember, ModifyGuildMembershipScreening,
        ModifyGuildMfaLevel, ModifyGuildOnboarding, ModifyGuildRole, ModifyGuildRolePosition,
        ModifyGuildWelcomeScreen, ModifyGuildWidget, SearchGuildMembers, VanityUrl,
    },
    voice::VoiceRegion,
};
use serde::Deserialize;

use super::PermissionsResponse;
use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

#[derive(Deserialize)]
struct PruneResponse {
    pruned: Option<u32>,
}

#[derive(Deserialize)]
struct MfaLevelResponse {
    level: MfaLevel,
}

impl Rest {
    pub fn get_guild<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a GetGuild,
    ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetGuild { guild_id }).query(query))
                .await
        })
    }

    pub fn get_guild_preview(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Preview, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildPreview { guild_id }))
                .await
        })
    }

    pub fn modify_guild<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuild,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuild { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn create_guild<'a>(
        &'a self,
        body: &'a CreateGuild,
    ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateGuild).json(body)?)
                .await
        })
    }

    pub fn delete_guild(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(Route::DeleteGuild { guild_id }))
                .await
        })
    }

    pub fn get_guild_member_verification(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<GuildMembershipScreening, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildMemberVerification { guild_id }))
                .await
        })
    }

    pub fn modify_guild_member_verification<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuildMembershipScreening,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<GuildMembershipScreening, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildMemberVerification { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_mfa_level<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuildMfaLevel,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<MfaLevel, Error>> + Send + 'a {
        boxed(async move {
            let response: MfaLevelResponse = self
                .call(
                    Request::new(Route::ModifyGuildMfaLevel { guild_id })
                        .json(body)?
                        .reason(reason),
                )
                .await?;
            Ok(response.level)
        })
    }

    pub fn get_guild_channels(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Channel>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildChannels { guild_id }))
                .await
        })
    }

    pub fn create_guild_channel<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildChannel,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildChannel { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_channel_positions<'a>(
        &'a self,
        guild_id: GuildId,
        positions: &'a [ModifyGuildChannelPosition],
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildChannelPositions { guild_id })
                    .json(positions)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn list_active_guild_threads(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<ActiveThreads, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ListActiveGuildThreads { guild_id }))
                .await
        })
    }
}

impl Rest {
    pub fn get_guild_member(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> impl Future<Output = Result<Member, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildMember { guild_id, user_id }))
                .await
        })
    }

    pub fn list_guild_members<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a ListGuildMembers,
    ) -> impl Future<Output = Result<Vec<Member>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListGuildMembers { guild_id }).query(query))
                .await
        })
    }

    pub fn search_guild_members<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a SearchGuildMembers,
    ) -> impl Future<Output = Result<Vec<Member>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::SearchGuildMembers { guild_id }).query(query))
                .await
        })
    }

    pub fn get_guild_member_permissions(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> impl Future<Output = Result<Permissions, Error>> + Send + '_ {
        boxed(async move {
            let response: PermissionsResponse = self
                .request(Request::new(Route::GetGuildMemberPermissions {
                    guild_id,
                    user_id,
                }))
                .await?;
            Ok(response.permissions)
        })
    }

    pub fn add_guild_member<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        body: &'a AddGuildMember,
    ) -> impl Future<Output = Result<Option<Member>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::AddGuildMember { guild_id, user_id }).json(body)?)
                .await
        })
    }

    pub fn modify_guild_member<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        body: &'a ModifyGuildMember,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Member, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildMember { guild_id, user_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_current_member<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyCurrentMember,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Member, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyCurrentMember { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_current_member_nickname<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyCurrentMemberNickname,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<ModifyCurrentMemberNickname, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyCurrentMemberNickname { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn add_guild_member_role<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::AddGuildMemberRole {
                    guild_id,
                    user_id,
                    role_id,
                })
                .reason(reason),
            )
            .await
        })
    }

    pub fn remove_guild_member_role<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        role_id: RoleId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::RemoveGuildMemberRole {
                    guild_id,
                    user_id,
                    role_id,
                })
                .reason(reason),
            )
            .await
        })
    }

    pub fn remove_guild_member<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::RemoveGuildMember { guild_id, user_id }).reason(reason))
                .await
        })
    }
}

impl Rest {
    pub fn get_guild_bans<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a GetGuildBans,
    ) -> impl Future<Output = Result<Vec<Ban>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetGuildBans { guild_id }).query(query))
                .await
        })
    }

    pub fn get_guild_ban(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> impl Future<Output = Result<Ban, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildBan { guild_id, user_id }))
                .await
        })
    }

    pub fn create_guild_ban<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        body: &'a CreateGuildBan,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildBan { guild_id, user_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn remove_guild_ban<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::RemoveGuildBan { guild_id, user_id }).reason(reason))
                .await
        })
    }

    pub fn bulk_guild_ban<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a BulkGuildBan,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<BulkBanResponse, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::BulkGuildBan { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }
}

impl Rest {
    pub fn get_guild_roles(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Role>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildRoles { guild_id }))
                .await
        })
    }

    pub fn get_guild_role(
        &self,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> impl Future<Output = Result<Role, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildRole { guild_id, role_id }))
                .await
        })
    }

    pub fn get_guild_role_permissions(
        &self,
        guild_id: GuildId,
        role_id: RoleId,
    ) -> impl Future<Output = Result<Permissions, Error>> + Send + '_ {
        boxed(async move {
            let response: PermissionsResponse = self
                .request(Request::new(Route::GetGuildRolePermissions {
                    guild_id,
                    role_id,
                }))
                .await?;
            Ok(response.permissions)
        })
    }

    pub fn get_guild_role_member_counts(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<HashMap<RoleId, u32>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildRoleMemberCounts { guild_id }))
                .await
        })
    }

    pub fn create_guild_role<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildRole,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Role, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildRole { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_role_positions<'a>(
        &'a self,
        guild_id: GuildId,
        positions: &'a [ModifyGuildRolePosition],
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Vec<Role>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildRolePositions { guild_id })
                    .json(positions)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_role<'a>(
        &'a self,
        guild_id: GuildId,
        role_id: RoleId,
        body: &'a ModifyGuildRole,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Role, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildRole { guild_id, role_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn delete_guild_role<'a>(
        &'a self,
        guild_id: GuildId,
        role_id: RoleId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteGuildRole { guild_id, role_id }).reason(reason))
                .await
        })
    }
}

impl Rest {
    pub fn get_guild_prune_count<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a GetGuildPruneCount,
    ) -> impl Future<Output = Result<u32, Error>> + Send + 'a {
        boxed(async move {
            let response: PruneResponse = self
                .request(Request::new(Route::GetGuildPruneCount { guild_id }).query(query))
                .await?;
            Ok(response.pruned.unwrap_or(0))
        })
    }

    pub fn begin_guild_prune<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a BeginGuildPrune,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Option<u32>, Error>> + Send + 'a {
        boxed(async move {
            let response: PruneResponse = self
                .request(
                    Request::new(Route::BeginGuildPrune { guild_id })
                        .json(body)?
                        .reason(reason),
                )
                .await?;
            Ok(response.pruned)
        })
    }

    pub fn get_guild_voice_regions(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<VoiceRegion>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildVoiceRegions { guild_id }))
                .await
        })
    }

    pub fn get_guild_invites(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<InviteWithMetadata>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildInvites { guild_id }))
                .await
        })
    }

    pub fn get_guild_integrations(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Integration>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildIntegrations { guild_id }))
                .await
        })
    }

    pub fn delete_guild_integration<'a>(
        &'a self,
        guild_id: GuildId,
        integration_id: IntegrationId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::DeleteGuildIntegration {
                    guild_id,
                    integration_id,
                })
                .reason(reason),
            )
            .await
        })
    }
}

impl Rest {
    pub fn get_guild_widget_settings(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<WidgetSettings, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildWidgetSettings { guild_id }))
                .await
        })
    }

    pub fn modify_guild_widget<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuildWidget,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<WidgetSettings, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildWidget { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn get_guild_widget(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Widget, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildWidget { guild_id }))
                .await
        })
    }

    pub fn get_guild_vanity_url(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<VanityUrl, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildVanityUrl { guild_id }))
                .await
        })
    }

    pub fn get_guild_welcome_screen(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<WelcomeScreen, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildWelcomeScreen { guild_id }))
                .await
        })
    }

    pub fn modify_guild_welcome_screen<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuildWelcomeScreen,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<WelcomeScreen, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildWelcomeScreen { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn get_guild_onboarding(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Onboarding, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildOnboarding { guild_id }))
                .await
        })
    }

    pub fn modify_guild_onboarding<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuildOnboarding,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Onboarding, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildOnboarding { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_incident_actions<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyGuildIncidentActions,
    ) -> impl Future<Output = Result<IncidentsData, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ModifyGuildIncidentActions { guild_id }).json(body)?)
                .await
        })
    }
}

use serde::Serialize;

use crate::model::{
    entitlement::EntitlementOwnerType,
    id::{EntitlementId, GenericId, GuildId, SkuId, UserId},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListEntitlements {
    pub user_id: Option<UserId>,

    pub sku_ids: Vec<SkuId>,

    pub before: Option<EntitlementId>,

    pub after: Option<EntitlementId>,

    pub limit: Option<u8>,

    pub guild_id: Option<GuildId>,

    pub exclude_ended: Option<bool>,

    pub exclude_deleted: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateTestEntitlement {
    pub sku_id: SkuId,

    pub owner_id: GenericId,

    pub owner_type: EntitlementOwnerType,
}

impl CreateTestEntitlement {
    #[must_use]
    pub const fn guild(sku_id: SkuId, guild_id: GuildId) -> Self {
        Self {
            sku_id,
            owner_id: guild_id.cast(),
            owner_type: EntitlementOwnerType::Guild,
        }
    }

    #[must_use]
    pub const fn user(sku_id: SkuId, user_id: UserId) -> Self {
        Self {
            sku_id,
            owner_id: user_id.cast(),
            owner_type: EntitlementOwnerType::User,
        }
    }
}

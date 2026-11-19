use serde::{Deserialize, Serialize};

use crate::{
    id::{ApplicationId, EntitlementId, GuildId, SkuId, UserId},
    macros::int_enum,
    util::Timestamp,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Entitlement {
    pub id: EntitlementId,

    pub sku_id: SkuId,

    pub application_id: ApplicationId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    #[serde(rename = "type")]
    pub kind: EntitlementType,

    pub deleted: bool,

    pub starts_at: Option<Timestamp>,

    pub ends_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub consumed: bool,
}

impl Entitlement {
    #[must_use]
    pub fn is_active(&self, now: Timestamp) -> bool {
        !self.deleted
            && !self.consumed
            && self.starts_at.is_none_or(|starts_at| starts_at <= now)
            && self.ends_at.is_none_or(|ends_at| now < ends_at)
    }
}

int_enum! {



    pub enum EntitlementType: u8 {

        Purchase = 1,

        PremiumSubscription = 2,

        DeveloperGift = 3,

        TestModePurchase = 4,

        FreePurchase = 5,

        UserGift = 6,

        PremiumPurchase = 7,

        ApplicationSubscription = 8,
    }
}

int_enum! {



    pub enum EntitlementOwnerType: u8 {

        Guild = 1,

        User = 2,
    }
}

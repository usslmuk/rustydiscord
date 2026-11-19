use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    id::{EntitlementId, SkuId, SubscriptionId, UserId},
    macros::int_enum,
    util::Timestamp,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Subscription {
    pub id: SubscriptionId,

    pub user_id: UserId,

    pub sku_ids: Vec<SkuId>,

    pub entitlement_ids: Vec<EntitlementId>,

    #[serde(default)]
    pub renewal_sku_ids: Option<Vec<SkuId>>,

    pub current_period_start: Timestamp,

    pub current_period_end: Timestamp,

    pub status: SubscriptionStatus,

    #[serde(default)]
    pub canceled_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<CompactString>,
}

int_enum! {



    pub enum SubscriptionStatus: u8 {

        Active = 0,

        Inactive = 1,

        Ending = 2,
    }
}

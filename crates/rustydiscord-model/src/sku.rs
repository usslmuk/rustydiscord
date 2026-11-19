use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    id::{ApplicationId, SkuId},
    macros::{int_enum, int_flags},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Sku {
    pub id: SkuId,

    #[serde(rename = "type")]
    pub kind: SkuType,

    pub application_id: ApplicationId,

    pub name: CompactString,

    pub slug: CompactString,

    pub flags: SkuFlags,
}

int_enum! {



    pub enum SkuType: u8 {

        Durable = 2,

        Consumable = 3,

        Subscription = 5,

        SubscriptionGroup = 6,
    }
}

int_flags! {



    pub struct SkuFlags: u64 {

        const AVAILABLE = 1 << 2;


        const GUILD_SUBSCRIPTION = 1 << 7;

        const USER_SUBSCRIPTION = 1 << 8;
    }
}

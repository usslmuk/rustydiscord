use crate::model::id::{SubscriptionId, UserId};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListSkuSubscriptions {
    pub before: Option<SubscriptionId>,

    pub after: Option<SubscriptionId>,

    pub limit: Option<u8>,

    pub user_id: Option<UserId>,
}

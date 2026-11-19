use serde::Serialize;

use crate::id::{ScheduledEventId, UserId};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetInvite {
    pub with_counts: Option<bool>,

    pub guild_scheduled_event_id: Option<ScheduledEventId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct BulkTargetUsers {
    pub user_ids: Vec<UserId>,
}

impl BulkTargetUsers {
    #[must_use]
    pub fn new(user_ids: impl IntoIterator<Item = UserId>) -> Self {
        Self {
            user_ids: user_ids.into_iter().collect(),
        }
    }
}

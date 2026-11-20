use crate::model::{
    audit_log::AuditLogEvent,
    id::{AuditLogEntryId, UserId},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetGuildAuditLog {
    pub user_id: Option<UserId>,

    pub action_type: Option<AuditLogEvent>,

    pub before: Option<AuditLogEntryId>,

    pub after: Option<AuditLogEntryId>,

    pub limit: Option<u8>,
}

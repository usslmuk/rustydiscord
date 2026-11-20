use crate::model::{audit_log::AuditLog, id::GuildId, rest::GetGuildAuditLog};

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_guild_audit_log<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a GetGuildAuditLog,
    ) -> impl Future<Output = Result<AuditLog, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetGuildAuditLog { guild_id }).query(query))
                .await
        })
    }
}

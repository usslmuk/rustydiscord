use crate::model::{
    id::UserId,
    invite::{Invite, InviteTargetUsersJobStatus},
    rest::{BulkTargetUsers, FileUpload, GetInvite},
};

use crate::rest::client::boxed;
use crate::rest::{Error, ErrorKind, Request, Rest, Route};

impl Rest {
    pub fn get_invite<'a>(
        &'a self,
        invite_code: &'a str,
        query: &'a GetInvite,
    ) -> impl Future<Output = Result<Invite, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetInvite { invite_code }).query(query))
                .await
        })
    }

    pub fn delete_invite<'a>(
        &'a self,
        invite_code: &'a str,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Invite, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteInvite { invite_code }).reason(reason))
                .await
        })
    }
}

impl Rest {
    pub fn get_target_users<'a>(
        &'a self,
        invite_code: &'a str,
    ) -> impl Future<Output = Result<Vec<UserId>, Error>> + Send + 'a {
        boxed(async move {
            let (_, body) = self
                .execute(Request::new(Route::GetTargetUsers { invite_code }))
                .await?;
            parse_target_users(&body)
        })
    }

    pub fn add_target_user<'a>(
        &'a self,
        invite_code: &'a str,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::AddTargetUser {
                invite_code,
                user_id,
            }))
            .await
        })
    }

    pub fn remove_target_user<'a>(
        &'a self,
        invite_code: &'a str,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::RemoveTargetUser {
                invite_code,
                user_id,
            }))
            .await
        })
    }

    pub fn update_target_users<'a>(
        &'a self,
        invite_code: &'a str,
        file: &'a FileUpload,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::UpdateTargetUsers { invite_code })
                    .form_file("target_users_file", file),
            )
            .await
        })
    }

    pub fn bulk_add_target_users<'a>(
        &'a self,
        invite_code: &'a str,
        body: &'a BulkTargetUsers,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::BulkAddTargetUsers { invite_code }).json(body)?)
                .await
        })
    }

    pub fn bulk_delete_target_users<'a>(
        &'a self,
        invite_code: &'a str,
        body: &'a BulkTargetUsers,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::BulkDeleteTargetUsers { invite_code }).json(body)?)
                .await
        })
    }

    pub fn get_target_users_job_status<'a>(
        &'a self,
        invite_code: &'a str,
    ) -> impl Future<Output = Result<InviteTargetUsersJobStatus, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetTargetUsersJobStatus { invite_code }))
                .await
        })
    }
}

fn parse_target_users(body: &[u8]) -> Result<Vec<UserId>, Error> {
    let text = core::str::from_utf8(body)
        .map_err(|source| Error::with_source(ErrorKind::Deserialize, source))?;
    let mut lines = text.lines().map(str::trim).enumerate().peekable();

    lines.next_if(|(_, line)| *line == "user_id");
    lines
        .filter(|(_, line)| !line.is_empty())
        .map(|(index, line)| {
            line.parse().map_err(|source| {
                Error::with_source(
                    ErrorKind::Deserialize,
                    format!(
                        "line {} of the target users file is not a user ID: {source}",
                        index + 1
                    ),
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_users_csv() {
        let ids = |body: &str| parse_target_users(body.as_bytes()).unwrap();
        assert_eq!(
            ids("user_id\n7\n8\n"),
            [UserId::new(7), UserId::new(8)],
            "header and trailing newline"
        );
        assert_eq!(ids("7\r\n\r\n8"), [UserId::new(7), UserId::new(8)]);
        assert_eq!(ids("user_id\n"), Vec::<UserId>::new());
        assert_eq!(ids(""), Vec::<UserId>::new());

        let error = parse_target_users(b"user_id\n7\nferris\n").unwrap_err();
        assert!(matches!(error.kind(), ErrorKind::Deserialize));
        assert!(
            core::error::Error::source(&error)
                .unwrap()
                .to_string()
                .starts_with("line 3 ")
        );
    }
}

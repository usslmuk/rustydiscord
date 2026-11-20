use crate::model::{
    application_identity_profile::{ApplicationIdentity, ApplicationIdentityProfile},
    id::{ApplicationId, UserId},
    rest::{
        DeleteApplicationIdentity, GetApplicationIdentitiesByExternalId,
        UpdateApplicationIdentityProfile,
    },
};
use serde::Deserialize;

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

#[derive(Deserialize)]
struct Identities {
    identities: Vec<ApplicationIdentity>,
}

impl Rest {
    pub fn update_application_identity_profile<'a>(
        &'a self,
        application_id: ApplicationId,
        user_id: UserId,
        provider_issued_user_id: &'a str,
        body: &'a UpdateApplicationIdentityProfile,
    ) -> impl Future<Output = Result<Option<ApplicationIdentityProfile>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::UpdateApplicationIdentityProfile {
                    application_id,
                    user_id,
                    provider_issued_user_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn get_application_identity_profile<'a>(
        &'a self,
        application_id: ApplicationId,
        user_id: UserId,
        provider_issued_user_id: &'a str,
    ) -> impl Future<Output = Result<ApplicationIdentityProfile, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetApplicationIdentityProfile {
                application_id,
                user_id,
                provider_issued_user_id,
            }))
            .await
        })
    }

    pub fn get_application_identities_by_user_id(
        &self,
        user_id: UserId,
        application_id: ApplicationId,
    ) -> impl Future<Output = Result<Vec<ApplicationIdentity>, Error>> + Send + '_ {
        boxed(async move {
            let identities: Identities = self
                .request(Request::new(Route::GetApplicationIdentitiesByUserId {
                    user_id,
                    application_id,
                }))
                .await?;
            Ok(identities.identities)
        })
    }

    pub fn get_application_identities_by_external_id<'a>(
        &'a self,
        application_id: ApplicationId,
        provider_type: &'a str,
        provider_issued_user_id: &'a str,
        query: &'a GetApplicationIdentitiesByExternalId,
    ) -> impl Future<Output = Result<Vec<ApplicationIdentity>, Error>> + Send + 'a {
        boxed(async move {
            let identities: Identities = self
                .request(
                    Request::new(Route::GetApplicationIdentitiesByExternalId {
                        application_id,
                        provider_type,
                        provider_issued_user_id,
                    })
                    .query(query),
                )
                .await?;
            Ok(identities.identities)
        })
    }

    pub fn delete_application_identity<'a>(
        &'a self,
        user_id: UserId,
        application_id: ApplicationId,
        provider_type: &'a str,
        provider_issued_user_id: &'a str,
        body: &'a DeleteApplicationIdentity,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::DeleteApplicationIdentity {
                    user_id,
                    application_id,
                    provider_type,
                    provider_issued_user_id,
                })
                .json(body)?,
            )
            .await
        })
    }
}

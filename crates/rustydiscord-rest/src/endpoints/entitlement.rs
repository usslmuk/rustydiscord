use rustydiscord_model::{
    entitlement::Entitlement,
    id::{ApplicationId, EntitlementId},
    rest::{CreateTestEntitlement, ListEntitlements},
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_entitlements<'a>(
        &'a self,
        application_id: ApplicationId,
        query: &'a ListEntitlements,
    ) -> impl Future<Output = Result<Vec<Entitlement>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListEntitlements { application_id }).query(query))
                .await
        })
    }

    pub fn get_entitlement(
        &self,
        application_id: ApplicationId,
        entitlement_id: EntitlementId,
    ) -> impl Future<Output = Result<Entitlement, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetEntitlement {
                application_id,
                entitlement_id,
            }))
            .await
        })
    }

    pub fn consume_entitlement(
        &self,
        application_id: ApplicationId,
        entitlement_id: EntitlementId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ConsumeEntitlement {
                application_id,
                entitlement_id,
            }))
            .await
        })
    }

    pub fn create_test_entitlement<'a>(
        &'a self,
        application_id: ApplicationId,
        body: &'a CreateTestEntitlement,
    ) -> impl Future<Output = Result<Entitlement, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateTestEntitlement { application_id }).json(body)?)
                .await
        })
    }

    pub fn delete_test_entitlement(
        &self,
        application_id: ApplicationId,
        entitlement_id: EntitlementId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteTestEntitlement {
                application_id,
                entitlement_id,
            }))
            .await
        })
    }
}

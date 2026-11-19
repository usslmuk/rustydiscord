use rustydiscord_model::{
    application_role_connection_metadata::ApplicationRoleConnectionMetadata, id::ApplicationId,
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_application_role_connection_metadata_records(
        &self,
        application_id: ApplicationId,
    ) -> impl Future<Output = Result<Vec<ApplicationRoleConnectionMetadata>, Error>> + Send + '_
    {
        boxed(async move {
            self.call(Request::new(
                Route::GetApplicationRoleConnectionMetadataRecords { application_id },
            ))
            .await
        })
    }

    pub fn update_application_role_connection_metadata_records<'a>(
        &'a self,
        application_id: ApplicationId,
        records: &'a [ApplicationRoleConnectionMetadata],
    ) -> impl Future<Output = Result<Vec<ApplicationRoleConnectionMetadata>, Error>> + Send + 'a
    {
        boxed(async move {
            self.call(
                Request::new(Route::UpdateApplicationRoleConnectionMetadataRecords {
                    application_id,
                })
                .json(records)?,
            )
            .await
        })
    }
}

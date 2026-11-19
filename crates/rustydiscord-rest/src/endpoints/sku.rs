use rustydiscord_model::{id::ApplicationId, sku::Sku};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_skus(
        &self,
        application_id: ApplicationId,
    ) -> impl Future<Output = Result<Vec<Sku>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ListSkus { application_id }))
                .await
        })
    }
}

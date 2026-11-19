use rustydiscord_model::{
    application::{ActivityInstance, Application},
    id::ApplicationId,
    rest::EditCurrentApplication,
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_current_application(
        &self,
    ) -> impl Future<Output = Result<Application, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::GetCurrentApplication)).await })
    }

    pub fn edit_current_application<'a>(
        &'a self,
        body: &'a EditCurrentApplication,
    ) -> impl Future<Output = Result<Application, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::EditCurrentApplication).json(body)?)
                .await
        })
    }

    pub fn get_application_activity_instance<'a>(
        &'a self,
        application_id: ApplicationId,
        instance_id: &'a str,
    ) -> impl Future<Output = Result<ActivityInstance, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetApplicationActivityInstance {
                application_id,
                instance_id,
            }))
            .await
        })
    }
}

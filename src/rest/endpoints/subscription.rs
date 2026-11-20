use crate::model::{
    id::{SkuId, SubscriptionId},
    rest::ListSkuSubscriptions,
    subscription::Subscription,
};

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_sku_subscriptions<'a>(
        &'a self,
        sku_id: SkuId,
        query: &'a ListSkuSubscriptions,
    ) -> impl Future<Output = Result<Vec<Subscription>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListSkuSubscriptions { sku_id }).query(query))
                .await
        })
    }

    pub fn get_sku_subscription(
        &self,
        sku_id: SkuId,
        subscription_id: SubscriptionId,
    ) -> impl Future<Output = Result<Subscription, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetSkuSubscription {
                sku_id,
                subscription_id,
            }))
            .await
        })
    }
}

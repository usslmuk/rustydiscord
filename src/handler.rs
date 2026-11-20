use std::{future::Future, marker::PhantomData, pin::Pin, sync::Arc};

use crate::cache::Backend;

use crate::{
    Context,
    event::{Dispatch, Event, EventKind},
    router::Routes,
};

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;

pub(crate) type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub(crate) trait ErasedHandler<B: Backend>: Send + Sync {
    fn spawn(&self, ctx: &Context<B>, dispatch: &Dispatch);

    fn call(&self, ctx: &Context<B>, dispatch: &Dispatch) -> Option<BoxFuture>;
}

pub(crate) struct Handler<E, F> {
    handler: F,
    event: PhantomData<fn() -> E>,
}

impl<E, F> Handler<E, F> {
    pub(crate) const fn new(handler: F) -> Self {
        Self {
            handler,
            event: PhantomData,
        }
    }
}

async fn run<E: Event>(future: impl Future<Output = Result<(), HandlerError>>) {
    if let Err(error) = future.await {
        tracing::error!(event = E::NAME, %error, "event handler returned an error");
    }
}

impl<B, E, F, Fut> ErasedHandler<B> for Handler<E, F>
where
    B: Backend,
    E: Event,
    F: Fn(Context<B>, Arc<E::Payload>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
{
    fn spawn(&self, ctx: &Context<B>, dispatch: &Dispatch) {
        if let Some(payload) = E::payload(dispatch) {
            tokio::spawn(run::<E>((self.handler)(ctx.clone(), Arc::clone(payload))));
        }
    }

    fn call(&self, ctx: &Context<B>, dispatch: &Dispatch) -> Option<BoxFuture> {
        let payload = E::payload(dispatch)?;
        Some(Box::pin(run::<E>((self.handler)(
            ctx.clone(),
            Arc::clone(payload),
        ))))
    }
}

pub(crate) struct Registry<B: Backend> {
    handlers: [Vec<Arc<dyn ErasedHandler<B>>>; EventKind::COUNT],

    pub(crate) routes: Routes<B>,
}

impl<B: Backend> Default for Registry<B> {
    fn default() -> Self {
        Self {
            handlers: core::array::from_fn(|_| Vec::new()),
            routes: Routes::default(),
        }
    }
}

impl<B: Backend> Clone for Registry<B> {
    fn clone(&self) -> Self {
        Self {
            handlers: self.handlers.clone(),
            routes: self.routes.clone(),
        }
    }
}

impl<B: Backend> Registry<B> {
    pub(crate) fn with(&self, kind: EventKind, handler: Arc<dyn ErasedHandler<B>>) -> Self {
        let mut registry = self.clone();
        registry.handlers[kind as usize].push(handler);
        registry
    }

    pub(crate) fn get(&self, kind: EventKind) -> &[Arc<dyn ErasedHandler<B>>] {
        &self.handlers[kind as usize]
    }
}

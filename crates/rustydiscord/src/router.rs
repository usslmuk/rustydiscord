use core::{fmt, ops::Deref};
use std::{collections::HashMap, future::Future, sync::Arc};

use rustydiscord_cache::Backend;
use rustydiscord_model::{
    gateway::payload::InteractionCreate,
    interaction::{CommandData, ComponentData, Interaction, InteractionData, ModalSubmitData},
};

use crate::{
    Context,
    handler::{BoxFuture, HandlerError},
};

macro_rules! wrapper {
    (
        $(#[$doc:meta])*
        $name:ident => $variant:ident($data:ty)
    ) => {
        $(#[$doc])*
        #[derive(Clone)]
        pub struct $name {
            event: Arc<InteractionCreate>,
        }

        impl $name {

            pub(crate) fn new(event: &Arc<InteractionCreate>) -> Option<Self> {
                matches!(event.data, InteractionData::$variant(_)).then(|| Self {
                    event: Arc::clone(event),
                })
            }




            #[must_use]
            pub fn interaction(&self) -> &Interaction {
                &self.event
            }





            #[must_use]
            pub const fn event(&self) -> &Arc<InteractionCreate> {
                &self.event
            }
        }

        impl Deref for $name {
            type Target = $data;

            fn deref(&self) -> &$data {
                match &self.event.data {
                    InteractionData::$variant(data) => data,

                    _ => unreachable!(concat!("a ", stringify!($name), " of another kind")),
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

                f.debug_tuple(stringify!($name)).field(self.interaction()).finish()
            }
        }
    };
}

wrapper! {







    CommandInteraction => ApplicationCommand(CommandData)
}

wrapper! {





    AutocompleteInteraction => Autocomplete(CommandData)
}

wrapper! {






    ComponentInteraction => MessageComponent(ComponentData)
}

wrapper! {






    ModalInteraction => ModalSubmit(ModalSubmitData)
}

fn args(custom_id: &str) -> &str {
    custom_id.split_once(':').map_or("", |(_, args)| args)
}

fn route_name(custom_id: &str) -> &str {
    custom_id
        .split_once(':')
        .map_or(custom_id, |(name, _)| name)
}

impl ComponentInteraction {
    #[must_use]
    pub fn args(&self) -> &str {
        args(&self.custom_id)
    }
}

impl ModalInteraction {
    #[must_use]
    pub fn args(&self) -> &str {
        args(&self.custom_id)
    }
}

pub(crate) trait ErasedRoute<B: Backend, W>: Send + Sync {
    fn spawn(&self, ctx: &Context<B>, interaction: W);

    fn call(&self, ctx: &Context<B>, interaction: W) -> BoxFuture;
}

pub(crate) struct Route<F> {
    name: Arc<str>,
    handler: F,
}

impl<F> Route<F> {
    pub(crate) fn new(name: &str, handler: F) -> Self {
        Self {
            name: name.into(),
            handler,
        }
    }
}

async fn run(route: Arc<str>, future: impl Future<Output = Result<(), HandlerError>>) {
    if let Err(error) = future.await {
        tracing::error!(route = &*route, %error, "interaction route handler returned an error");
    }
}

impl<B, W, F, Fut> ErasedRoute<B, W> for Route<F>
where
    B: Backend,
    F: Fn(Context<B>, W) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
{
    fn spawn(&self, ctx: &Context<B>, interaction: W) {
        let future = (self.handler)(ctx.clone(), interaction);
        tokio::spawn(run(Arc::clone(&self.name), future));
    }

    fn call(&self, ctx: &Context<B>, interaction: W) -> BoxFuture {
        let future = (self.handler)(ctx.clone(), interaction);
        Box::pin(run(Arc::clone(&self.name), future))
    }
}

type RouteMap<B, W> = HashMap<Box<str>, Arc<dyn ErasedRoute<B, W>>>;

pub(crate) struct Routes<B: Backend> {
    pub(crate) commands: RouteMap<B, CommandInteraction>,
    pub(crate) autocomplete: RouteMap<B, AutocompleteInteraction>,
    pub(crate) components: RouteMap<B, ComponentInteraction>,
    pub(crate) modals: RouteMap<B, ModalInteraction>,
}

impl<B: Backend> Default for Routes<B> {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
            autocomplete: HashMap::new(),
            components: HashMap::new(),
            modals: HashMap::new(),
        }
    }
}

impl<B: Backend> Clone for Routes<B> {
    fn clone(&self) -> Self {
        Self {
            commands: self.commands.clone(),
            autocomplete: self.autocomplete.clone(),
            components: self.components.clone(),
            modals: self.modals.clone(),
        }
    }
}

enum Found<'a, B: Backend> {
    Command(
        &'a dyn ErasedRoute<B, CommandInteraction>,
        CommandInteraction,
    ),
    Autocomplete(
        &'a dyn ErasedRoute<B, AutocompleteInteraction>,
        AutocompleteInteraction,
    ),
    Component(
        &'a dyn ErasedRoute<B, ComponentInteraction>,
        ComponentInteraction,
    ),
    Modal(&'a dyn ErasedRoute<B, ModalInteraction>, ModalInteraction),
}

impl<B: Backend> Routes<B> {
    pub(crate) fn is_empty(&self) -> bool {
        self.commands.is_empty()
            && self.autocomplete.is_empty()
            && self.components.is_empty()
            && self.modals.is_empty()
    }

    pub(crate) fn spawn(&self, ctx: &Context<B>, event: &Arc<InteractionCreate>) {
        match self.find(event) {
            Some(Found::Command(route, interaction)) => route.spawn(ctx, interaction),
            Some(Found::Autocomplete(route, interaction)) => route.spawn(ctx, interaction),
            Some(Found::Component(route, interaction)) => route.spawn(ctx, interaction),
            Some(Found::Modal(route, interaction)) => route.spawn(ctx, interaction),
            None => {}
        }
    }

    pub(crate) fn call(
        &self,
        ctx: &Context<B>,
        event: &Arc<InteractionCreate>,
    ) -> Option<BoxFuture> {
        Some(match self.find(event)? {
            Found::Command(route, interaction) => route.call(ctx, interaction),
            Found::Autocomplete(route, interaction) => route.call(ctx, interaction),
            Found::Component(route, interaction) => route.call(ctx, interaction),
            Found::Modal(route, interaction) => route.call(ctx, interaction),
        })
    }

    fn find(&self, event: &Arc<InteractionCreate>) -> Option<Found<'_, B>> {
        Some(match &event.data {
            InteractionData::ApplicationCommand(data) => Found::Command(
                &**command_route(&self.commands, data)?,
                CommandInteraction::new(event)?,
            ),
            InteractionData::Autocomplete(data) => Found::Autocomplete(
                &**command_route(&self.autocomplete, data)?,
                AutocompleteInteraction::new(event)?,
            ),
            InteractionData::MessageComponent(data) => Found::Component(
                &**self.components.get(route_name(&data.custom_id))?,
                ComponentInteraction::new(event)?,
            ),
            InteractionData::ModalSubmit(data) => Found::Modal(
                &**self.modals.get(route_name(&data.custom_id))?,
                ModalInteraction::new(event)?,
            ),
            _ => return None,
        })
    }
}

fn command_route<'a, B: Backend, W>(
    routes: &'a RouteMap<B, W>,
    data: &CommandData,
) -> Option<&'a Arc<dyn ErasedRoute<B, W>>> {
    if routes.is_empty() {
        return None;
    }
    let mut path = [data.name.as_str(), "", ""];
    let mut len = 1;
    for part in [data.subcommand_group(), data.subcommand()]
        .into_iter()
        .flatten()
    {
        path[len] = part;
        len += 1;
    }
    (2..=len)
        .rev()
        .find_map(|len| joined(&path[..len], |key| routes.get(key)))
        .or_else(|| routes.get(path[0]))
}

fn joined<R>(parts: &[&str], f: impl FnOnce(&str) -> R) -> R {
    let len = parts.iter().map(|part| part.len() + 1).sum::<usize>() - 1;
    let mut buffer = [0; 256];
    if len > buffer.len() {
        return f(&parts.join(" "));
    }
    let mut at = 0;
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            buffer[at] = b' ';
            at += 1;
        }
        buffer[at..at + part.len()].copy_from_slice(part.as_bytes());
        at += part.len();
    }

    f(core::str::from_utf8(&buffer[..at]).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_ids_are_split_at_the_first_colon() {
        assert_eq!((route_name("vote"), args("vote")), ("vote", ""));
        assert_eq!((route_name("vote:42"), args("vote:42")), ("vote", "42"));
        assert_eq!((route_name("vote:a:b"), args("vote:a:b")), ("vote", "a:b"));
        assert_eq!((route_name(":x"), args(":x")), ("", "x"));
    }

    #[test]
    fn paths_are_joined_on_the_stack_or_the_heap() {
        assert_eq!(
            joined(&["mod", "config", "set"], str::to_owned),
            "mod config set"
        );
        assert_eq!(joined(&["ping"], str::to_owned), "ping");
        let long = "ż".repeat(100);
        assert_eq!(
            joined(&[&long, &long], str::to_owned),
            format!("{long} {long}")
        );
    }
}

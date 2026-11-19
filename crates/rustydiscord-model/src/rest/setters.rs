use crate::{message::MessageFlags, util::Patch};

pub(crate) trait Slot<T> {
    fn put(&mut self, value: T);

    fn value(&self) -> Option<&T>;
}

impl<T> Slot<T> for Option<T> {
    fn put(&mut self, value: T) {
        *self = Some(value);
    }

    fn value(&self) -> Option<&T> {
        self.as_ref()
    }
}

impl<T> Slot<T> for Patch<T> {
    fn put(&mut self, value: T) {
        *self = Self::Set(value);
    }

    fn value(&self) -> Option<&T> {
        match self {
            Self::Set(value) => Some(value),
            Self::Keep | Self::Clear => None,
        }
    }
}

pub(crate) trait List<T> {
    fn push_item(&mut self, item: T);

    fn replace(&mut self, items: Vec<T>);
}

impl<T> List<T> for Vec<T> {
    fn push_item(&mut self, item: T) {
        self.push(item);
    }

    fn replace(&mut self, items: Vec<T>) {
        *self = items;
    }
}

impl<T> List<T> for Option<Vec<T>> {
    fn push_item(&mut self, item: T) {
        self.get_or_insert_with(Vec::new).push(item);
    }

    fn replace(&mut self, items: Vec<T>) {
        *self = Some(items);
    }
}

impl<T> List<T> for Patch<Vec<T>> {
    fn push_item(&mut self, item: T) {
        match self {
            Self::Set(items) => items.push(item),
            Self::Keep | Self::Clear => *self = Self::Set(vec![item]),
        }
    }

    fn replace(&mut self, items: Vec<T>) {
        *self = Self::Set(items);
    }
}

pub(crate) fn insert_flags(slot: &mut impl Slot<MessageFlags>, flags: MessageFlags) {
    let current = slot.value().copied().unwrap_or_default();
    slot.put(current | flags);
}

macro_rules! message_body {
    ($body:ident { $($setter:ident),* $(,)? }) => {
        impl $body {
            $($crate::rest::setters::message_body!(@ $setter);)*
        }

        impl From<&str> for $body {
            fn from(content: &str) -> Self {
                Self::default().content(content)
            }
        }

        impl From<String> for $body {
            fn from(content: String) -> Self {
                Self::default().content(content)
            }
        }

        impl From<::compact_str::CompactString> for $body {
            fn from(content: ::compact_str::CompactString) -> Self {
                Self::default().content(content)
            }
        }
    };
    (@ content) => {



        #[must_use]
        pub fn content(mut self, content: impl Into<::compact_str::CompactString>) -> Self {
            $crate::rest::setters::Slot::put(&mut self.content, content.into());
            self
        }
    };
    (@ embeds) => {




        #[must_use]
        pub fn embed(mut self, embed: $crate::message::Embed) -> Self {
            $crate::rest::setters::List::push_item(&mut self.embeds, embed);
            self
        }





        #[must_use]
        pub fn embeds(
            mut self,
            embeds: impl IntoIterator<Item = $crate::message::Embed>,
        ) -> Self {
            $crate::rest::setters::List::replace(&mut self.embeds, embeds.into_iter().collect());
            self
        }
    };
    (@ components) => {





        #[must_use]
        pub fn component(mut self, component: impl Into<$crate::component::Component>) -> Self {
            $crate::rest::setters::List::push_item(&mut self.components, component.into());
            self
        }






        #[must_use]
        pub fn components(
            mut self,
            components: impl IntoIterator<Item = $crate::component::Component>,
        ) -> Self {
            $crate::rest::setters::List::replace(
                &mut self.components,
                components.into_iter().collect(),
            );
            self
        }
    };
    (@ allowed_mentions) => {



        #[must_use]
        pub fn allowed_mentions(mut self, mentions: $crate::message::AllowedMentions) -> Self {
            $crate::rest::setters::Slot::put(&mut self.allowed_mentions, mentions);
            self
        }





        #[must_use]
        pub fn no_mentions(self) -> Self {
            self.allowed_mentions($crate::message::AllowedMentions::default())
        }
    };
    (@ flags) => {




        #[must_use]
        pub fn flags(mut self, flags: $crate::message::MessageFlags) -> Self {
            $crate::rest::setters::Slot::put(&mut self.flags, flags);
            self
        }





        #[must_use]
        pub fn suppress_embeds(mut self) -> Self {
            $crate::rest::setters::insert_flags(
                &mut self.flags,
                $crate::message::MessageFlags::SUPPRESS_EMBEDS,
            );
            self
        }
    };
    (@ ephemeral) => {




        #[must_use]
        pub fn ephemeral(mut self) -> Self {
            $crate::rest::setters::insert_flags(
                &mut self.flags,
                $crate::message::MessageFlags::EPHEMERAL,
            );
            self
        }
    };
    (@ suppress_notifications) => {





        #[must_use]
        pub fn suppress_notifications(mut self) -> Self {
            $crate::rest::setters::insert_flags(
                &mut self.flags,
                $crate::message::MessageFlags::SUPPRESS_NOTIFICATIONS,
            );
            self
        }
    };
    (@ files) => {



        #[must_use]
        pub fn file(mut self, file: $crate::rest::FileUpload) -> Self {
            self.files.push(file);
            self
        }




        #[must_use]
        pub fn files(mut self, files: impl IntoIterator<Item = $crate::rest::FileUpload>) -> Self {
            self.files = files.into_iter().collect();
            self
        }
    };
    (@ attachments) => {




        #[must_use]
        pub fn attachments(
            mut self,
            attachments: impl IntoIterator<Item = $crate::message::AttachmentRequest>,
        ) -> Self {
            $crate::rest::setters::List::replace(
                &mut self.attachments,
                attachments.into_iter().collect(),
            );
            self
        }
    };
    (@ poll) => {



        #[must_use]
        pub fn poll(mut self, poll: $crate::poll::CreatePoll) -> Self {
            $crate::rest::setters::Slot::put(&mut self.poll, poll);
            self
        }
    };
    (@ tts) => {



        #[must_use]
        pub const fn tts(mut self, tts: bool) -> Self {
            self.tts = tts;
            self
        }
    };
}

pub(crate) use message_body;

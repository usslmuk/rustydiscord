use compact_str::CompactString;
use rustydiscord_model::component::{
    File, MediaGallery, MediaGalleryItem, TextDisplay, Thumbnail, UnfurledMediaItem,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct TextDisplayBuilder {
    component: TextDisplay,
}

impl TextDisplayBuilder {
    pub fn new(content: impl Into<CompactString>) -> Self {
        Self {
            component: TextDisplay::new(content),
        }
    }
}

component_builder!(TextDisplayBuilder => TextDisplay);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a section or built"]
pub struct ThumbnailBuilder {
    component: Thumbnail,
}

impl ThumbnailBuilder {
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self {
            component: Thumbnail::new(UnfurledMediaItem::new(url)),
        }
    }

    pub fn description(mut self, description: impl Into<CompactString>) -> Self {
        self.component.description = Some(description.into());
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.component.spoiler = spoiler;
        self
    }
}

component_builder!(ThumbnailBuilder => Thumbnail);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct MediaGalleryBuilder {
    component: MediaGallery,
}

impl Default for MediaGalleryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaGalleryBuilder {
    pub fn new() -> Self {
        Self {
            component: MediaGallery::new(Vec::new()),
        }
    }

    pub fn item(mut self, url: impl Into<CompactString>) -> Self {
        self.component
            .items
            .push(MediaGalleryItem::new(UnfurledMediaItem::new(url)));
        self
    }

    pub fn item_with(mut self, item: MediaGalleryItemBuilder) -> Self {
        self.component.items.push(item.item);
        self
    }

    pub fn items<I: Into<MediaGalleryItem>>(mut self, items: impl IntoIterator<Item = I>) -> Self {
        self.component
            .items
            .extend(items.into_iter().map(Into::into));
        self
    }
}

component_builder!(MediaGalleryBuilder => MediaGallery);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "an item does nothing until it is added to a gallery"]
pub struct MediaGalleryItemBuilder {
    item: MediaGalleryItem,
}

impl MediaGalleryItemBuilder {
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self {
            item: MediaGalleryItem::new(UnfurledMediaItem::new(url)),
        }
    }

    pub fn description(mut self, description: impl Into<CompactString>) -> Self {
        self.item.description = Some(description.into());
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.item.spoiler = spoiler;
        self
    }
}

impl From<MediaGalleryItemBuilder> for MediaGalleryItem {
    fn from(item: MediaGalleryItemBuilder) -> Self {
        item.item
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct FileBuilder {
    component: File,
}

impl FileBuilder {
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self {
            component: File::new(UnfurledMediaItem::new(url)),
        }
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.component.spoiler = spoiler;
        self
    }
}

component_builder!(FileBuilder => File);

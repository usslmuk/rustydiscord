#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod iter;
mod map;
mod random;

pub use self::map::{Collection, DefaultHashBuilder};

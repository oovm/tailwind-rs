use crate::{Result, TailwindBuilder, TailwindInstance};
use base64::{encode_config, URL_SAFE_NO_PAD};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
};
use xxhash_rust::xxh3::Xxh3;

pub use self::{attribute::CssAttributes, behavior::CssBehavior, bundle::CssBundle, instance::CssInstance};
mod attribute;
mod behavior;
mod bundle;
mod instance;

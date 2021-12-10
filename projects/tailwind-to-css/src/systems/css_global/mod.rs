use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
};

use crate::{Result, TailwindBuilder, TailwindInstance};

pub use self::{attribute::CssAttribute, behavior::CssBehavior, bundle::CssBundle, instance::CssInstance, mode::InlineMode};
mod attribute;
mod behavior;
mod bundle;
mod instance;
mod mode;

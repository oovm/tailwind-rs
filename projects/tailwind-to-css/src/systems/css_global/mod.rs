use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter, Write},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign},
};

use base64::{encode_config, URL_SAFE_NO_PAD};
use itertools::Itertools;
use xxhash_rust::xxh3::Xxh3;

use crate::{Result, TailwindBuilder, TailwindInstance};

pub use self::{attribute::CssAttributes, mode::CssInlineMode};
pub(crate) use self::{bundle::CssBundle, instance::CssInstance};

mod attribute;
mod bundle;
mod instance;
mod mode;

fn normalize_class_name(f: &mut (dyn Write), name: &str) -> Result<()> {
    for c in name.chars() {
        match c {
            ' ' => write!(f, "_"),
            r @ ('-' | '_') => write!(f, "{}", r),
            a if a.is_alphanumeric() => write!(f, "{}", a),
            _ => write!(f, "\\{}", c),
        }?
    }
    Ok(())
}

fn base64(hash: u64) -> String {
    format!("_{}", encode_config(hash.to_be_bytes(), URL_SAFE_NO_PAD))
}

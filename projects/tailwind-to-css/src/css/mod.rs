mod resolve_normal;
mod resolve_ring;
use crate::LengthUnit;
use css_color_parser::Color;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

mod attribute;

pub struct CssApplyResolver {
    apply: BTreeMap<String, CssAttributes>,
}

pub struct CssAttributes {
    apply: Option<String>,
    normal: BTreeMap<String, String>,
    transforms: BTreeMap<String, String>,
    scoped: BTreeMap<String, CssAttributes>,
    ring_resolver: Option<CssRingResolver>,
}

pub struct CssRingResolver {
    tw_ring_inset: String,
    tw_ring_offset_width: String,
    tw_ring_color: String,
    tw_ring_offset_color: String,
    tw_ring_shadow: String,
}

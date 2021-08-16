use crate::{css_attributes, CssAttribute, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

#[doc = include_str!("readme.md")]
#[derive(Debug)]
pub struct TailwindScreenReader {
    sr_only: bool,
}

impl Display for TailwindScreenReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.sr_only {
            f.write_str("not-")?
        }
        f.write_str("sr-only")
    }
}

impl TailwindInstance for TailwindScreenReader {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self.sr_only {
            true => {
                css_attributes! {
                    "position" => "absolute",
                    "width" => "1px",
                    "height" => "1px",
                    "padding" => "0",
                    "margin" => "-1px",
                    "overflow" => "hidden",
                    "clip" => "rect(0,0,0,0)",
                    "white-space" => "nowrap",
                    "border-width" => "0",

                }
            }
            false => {
                css_attributes! {
                    "position" => "static",
                    "width" => "auto",
                    "height" => "auto",
                    "padding" => "0",
                    "margin" => "0",
                    "overflow" => "visible",
                    "clip" => "auto",
                    "white-space" => "normal",

                }
            }
        }
    }
}

impl TailwindScreenReader {
    #[inline]
    pub fn new(sr_only: bool) -> Self {
        Self { sr_only }
    }
}

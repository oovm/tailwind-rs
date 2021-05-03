mod builder;
mod parser;

use super::*;

#[doc = include_str ! ("border-collapse.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailwindBorderCollapse {
    Collapse,
    Separate,
}

#[doc = include_str ! ("table-layout.md")]
#[derive(Copy, Clone, Debug)]
pub enum TailwindTableLayout {
    Auto,
    Fixed,
}

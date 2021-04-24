use super::*;

pub mod builder;
pub mod parser;

/// https://tailwindcss.com/docs/aspect-ratio
pub struct LayoutSystem {}

//         TailwindObject::parser("aspect-auto", "aspect-ratio: auto;"),
//         TailwindObject::parser("aspect-square", "aspect-ratio: 1 / 1;"),
//         TailwindObject::parser("aspect-video", "aspect-ratio: 16 / 9;"),

/// https://tailwindcss.com/docs/aspect-ratio
///
/// ```
/// use tailwind_css::TailwindAspect;
/// ```
pub struct TailwindAspect {
    kind: &'static str,
    ratio: &'static str,
}

pub enum TailwindBreak {
    /// https://tailwindcss.com/docs/break-before
    Before(&'static str),
    /// https://tailwindcss.com/docs/break-after
    After(&'static str),
    /// https://tailwindcss.com/docs/break-inside
    Inside(&'static str),
}

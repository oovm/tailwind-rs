mod font_family;
mod font_size;
mod font_smoothing;
mod font_style;
mod font_variant_numeric;
mod font_weight;

pub use self::{
    font_family::TailwindFontFamily, font_style::TailwindFontStyle, font_variant_numeric::TailwindFontVariantNumeric,
};

use super::*;

/// font that unknown at parsing time
#[derive(Debug, Clone)]
pub struct TailwindFont {
    arbitrary: String,
}

pub fn font_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/float
        [s @ ("sans" | "serif" | "mono")] => TailwindFontFamily::new(s).boxed(),
        // https://tailwindcss.com/docs/font-size
        ["xs"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["sm"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["md"] | ["base"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["lg"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["2xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["3xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["4xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["5xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["6xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["7xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["8xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        ["9xl"] => TailwindFontSize::new(0.75, 1.0).boxed(),
        // https://tailwindcss.com/docs/float
        ["thin"] => TailwindFontWeight::THIN.boxed(),
        ["extralight"] => TailwindFontWeight::EXTRA_LIGHT.boxed(),
        ["light"] => TailwindFontWeight::LIGHT.boxed(),
        ["normal"] => TailwindFontWeight::NORMAL.boxed(),
        ["medium"] => TailwindFontWeight::MEDIUM.boxed(),
        ["semibold"] => TailwindFontWeight::SEMI_BOLD.boxed(),
        ["bold"] => TailwindFontWeight::BOLD.boxed(),
        ["extrabold"] => TailwindFontWeight::EXTRA_BOLD.boxed(),
        ["black"] => TailwindFontWeight::BLACK.boxed(),
        _ => return syntax_error!("Unknown font instructions: {}", str.join("-")),
    };
    Ok(out)
}

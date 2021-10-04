use super::*;

pub use self::{
    font_family::TailwindFontFamily, font_size::TailwindFontSize, font_smoothing::TailwindFontSmoothing,
    font_style::TailwindFontStyle, font_variant_numeric::TailwindFontVariantNumeric, font_weight::TailwindFontWeight,
};

mod font_family;
mod font_size;
mod font_smoothing;
mod font_style;
mod font_variant_numeric;
mod font_weight;

pub fn font_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
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
        ["size"] => maybe_size(arbitrary)?,
        ["size", n] => {
            let a = TailwindArbitrary::from(*n);
            maybe_size(&a)?
        },
        [n] => {
            let a = TailwindArbitrary::from(*n);
            maybe_weight(&a).or_else(|_| maybe_size(&a))?
        },
        _ => TailwindFontFamily::from(pattern.join("-")).boxed(),
    };
    Ok(out)
}

fn maybe_weight(arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let w = arbitrary.as_integer()?;
    Ok(TailwindFontWeight::new(w).boxed())
}

fn maybe_size(arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let w = arbitrary.as_integer()?;
    Ok(TailwindFontWeight::new(w).boxed())
}

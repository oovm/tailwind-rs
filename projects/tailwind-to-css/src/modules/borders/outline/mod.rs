use super::*;

pub(crate) mod outline_color;
pub(crate) mod outline_offset;
pub(crate) mod outline_style;
pub(crate) mod outline_width;

pub(crate) fn outline_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/outline-style
        [] if arbitrary.is_none() => TailwindOutlineStyle::from("solid").boxed(),
        [s @ ("dashed" | "dotted" | "double" | "hidden" | "solid")] => TailwindOutlineStyle::from(*s).boxed(),
        ["none"] => TailwindOutlineStyle::from("<NONE>").boxed(),
        ["style", rest @ ..] => TailwindOutlineStyle::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/outline-offset
        ["offset", rest @ ..] => TailwindOutlineOffset::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/outline-width
        ["width", rest @ ..] => TailwindOutlineWidth::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/outline-color
        ["black" | "white" | "inherit" | "current" | "transparent" | "revert"] =>
            TailwindOutlineColor::parse(pattern, arbitrary)?.boxed(),
        ["color", rest @ ..] => TailwindOutlineColor::parse(rest, arbitrary)?.boxed(),
        // Flexible parsing pattern
        [] if arbitrary.as_str().starts_with(|c: char| c.is_numeric()) => TailwindOutlineWidth::from(arbitrary).boxed(),
        [n] => resolve1(n)?,
        _ => TailwindOutlineColor::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(out)
}

fn resolve1(n: &str) -> Result<Box<dyn TailwindInstance>> {
    let a = TailwindArbitrary::from(n);
    if n.starts_with(|c: char| c.is_numeric()) {
        return Ok(resolve1_length(&a).or_else(|_| resolve1_unit(&a))?.boxed());
    }
    if n.starts_with(|c: char| c == '#') {
        return Ok(resolve1_color(&a)?.boxed());
    }
    Ok(TailwindOutlineColor::from(TailwindColor::Themed(n.to_string(), 0)).boxed())
}

fn resolve1_length(a: &TailwindArbitrary) -> Result<TailwindOutlineWidth> {
    Ok(TailwindOutlineWidth::from(a.as_length()?))
}

fn resolve1_unit(a: &TailwindArbitrary) -> Result<TailwindOutlineWidth> {
    Ok(TailwindOutlineWidth::from(a.as_integer()?))
}

fn resolve1_color(a: &TailwindArbitrary) -> Result<TailwindOutlineColor> {
    Ok(TailwindOutlineColor::from(a.as_color()?))
}

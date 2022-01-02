use super::*;

pub(crate) mod outline_color;
pub(crate) mod outline_offset;
pub(crate) mod outline_style;
pub(crate) mod outline_width;

pub(crate) fn outline_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/outline-style
        [] if arbitrary.is_none() => TailwindOutlineStyle::from("solid").boxed(),
        [s @ ("dashed" | "dotted" | "double" | "hidden")] => TailwindOutlineStyle::from(*s).boxed(),
        ["none"] => TailwindOutlineStyle::from("<NONE>").boxed(),
        ["style", rest @ ..] => TailwindOutlineStyle::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/outline-offset
        ["offset", rest @ ..] => TailwindOutlineOffset::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/outline-width
        [n] => resolve1(n)?,
        _ => TailwindOutlineColor::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(out)
}

fn resolve1(n: &str) -> Result<Box<dyn TailwindInstance>> {
    let a = TailwindArbitrary::from(n);
    if n.starts_with(|c: char| c.is_numeric()) {
        if cfg!(compile_time) && n.contains('/') {
            return syntax_error!("outline-width can't be fraction");
        }
        return Ok(TailwindOutlineWidth::from(a.as_length_or_fraction()?).boxed());
    }
    if n.starts_with(|c: char| c == '#') {
        return Ok(TailwindOutlineColor::from(a.as_color()?).boxed());
    }
    syntax_error!("outline-width can't be {}", n)
}

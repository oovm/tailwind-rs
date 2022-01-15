use super::*;

pub(crate) mod snap_align;
pub(crate) mod snap_stop;
pub(crate) mod snap_type;

pub(crate) fn snap_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/scroll-snap-align
        [s @ ("start" | "end" | "center")] => TailwindSnapAlign::from(*s).boxed(),
        ["align", rest @ ..] => TailwindSnapAlign::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/scroll-snap-stop
        [s @ ("normal" | "always")] => TailwindSnapStop::from(*s).boxed(),
        ["stop", rest @ ..] => TailwindSnapStop::parse(rest, arbitrary)?.boxed(),
        _ => TailwindSnapType::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(out)
}

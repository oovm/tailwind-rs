use super::*;

pub(crate) mod scroll_behavior;

pub(crate) fn scroll_adaptor(
    pattern: &[&str],
    arbitrary: &TailwindArbitrary,
    negative: Negative,
) -> Result<Box<dyn TailwindInstance>> {
    let kind = match pattern {
        ["p" | "pl" | "pr" | "pb" | "pt" | "px" | "py", ..] =>
            TailwindScrollPadding::parse(pattern, arbitrary, negative)?.boxed(),
        ["m" | "ml" | "mr" | "mb" | "mt" | "mx" | "my", ..] =>
            TailwindScrollMargin::parse(pattern, arbitrary, negative)?.boxed(),
        _ => TailwindScrollBehavior::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(kind)
}

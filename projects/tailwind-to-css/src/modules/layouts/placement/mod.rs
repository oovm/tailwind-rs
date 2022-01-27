use super::*;

pub use self::{bottom::TailwindBottom, inset::TailwindInset, left::TailwindLeft, right::TailwindRight, top::TailwindTop};

mod bottom;
mod inset;
mod left;
mod right;
mod top;

pub(crate) fn get_kind_px_full_auto_fact(
    id: &'static str,
    pattern: &[&str],
    arbitrary: &TailwindArbitrary,
    negative: Negative,
) -> Result<UnitValue> {
    let kind = match pattern {
        ["px"] => UnitValue::px(1.0),
        ["full"] => UnitValue::radio(1, 1),
        _ => UnitValue::negative_parser(id, check_valid_auto, true, false, true)(pattern, arbitrary, negative)?,
    };
    Ok(kind)
}

pub(crate) fn check_valid_auto(mode: &str) -> bool {
    ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
}

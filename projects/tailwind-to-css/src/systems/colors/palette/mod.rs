use super::*;
mod builtin;

/// In general, it is a look-up table.
///
/// When the color is automatically mixed when the interpolation expression is activated.
#[derive(Clone, Debug)]
pub struct Palette {
    /// Allow gradients?
    gradient: bool,
    /// min-width
    /// unit: px
    key_points: BTreeMap<usize, Srgb>,
}

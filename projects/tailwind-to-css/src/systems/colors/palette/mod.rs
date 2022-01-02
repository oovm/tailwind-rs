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
    key_points: BTreeMap<u32, Srgb>,
}

impl Palette {
    ///
    pub fn get_color(&self, weight: u32) -> Result<Srgb> {
        match self.key_points.get(&weight) {
            Some(s) => Ok(*s),
            None if self.gradient => {
                syntax_error!("TODO")
            },
            None => {
                syntax_error!("TODO")
            },
        }
    }
}

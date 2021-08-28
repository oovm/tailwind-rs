use super::*;

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindBorderStyle {
    kind: BorderStyle,
}

#[derive(Copy, Clone, Debug)]
enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Hidden,
}

// border-solid	border-style: solid;
// border-dashed	border-style: dashed;
// border-dotted	border-style: dotted;
// border-double	border-style: double;
// border-hidden	border-style: hidden;
// border-none	border-style: none;
impl Display for TailwindBorderStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBorderStyle {
    fn attributes(&self, ctx: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        todo!()
    }
}

// border-solid	border-style: solid;
// border-dashed	border-style: dashed;
// border-dotted	border-style: dotted;
// border-double	border-style: double;
// border-hidden	border-style: hidden;
// border-none	border-style: none;
impl TailwindBorderStyle {
    /// `tracking-normal`
    pub const Normal: Self = Self { kind: BorderStyle::None };
    /// `tracking-inherit`
    pub const Inherit: Self = Self { kind: BorderStyle::None };
    /// `tracking-initial`
    pub const Initial: Self = Self { kind: BorderStyle::None };
    /// `tracking-unset`
    pub const Unset: Self = Self { kind: BorderStyle::None };
    /// `tracking-initial`
    pub const Initial: Self = Self { kind: BorderStyle::None };
    /// `tracking-unset`
    pub const Unset: Self = Self { kind: BorderStyle::None };
}

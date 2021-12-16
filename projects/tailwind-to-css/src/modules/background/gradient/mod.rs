use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindFrom {
    color: TailwindColor,
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindVia {
    color: TailwindColor,
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTo {
    color: TailwindColor,
}
crate::macros::sealed::color_instance!(TailwindFrom);
crate::macros::sealed::color_instance!(TailwindVia);
crate::macros::sealed::color_instance!(TailwindTo);

impl Display for TailwindFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "from-{}", self.color)
    }
}
impl Display for TailwindVia {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "via-{}", self.color)
    }
}
impl Display for TailwindTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "to-{}", self.color)
    }
}
impl TailwindInstance for TailwindFrom {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "--tw-gradient-from" => color,
            "--tw-gradient-stops" => format!("var(--tw-gradient-from),var(--tw-gradient-to,{color})", color=color)
        }
    }
}
impl TailwindInstance for TailwindVia {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "--tw-gradient-stops" => format!("var(--tw-gradient-from),{color},var(--tw-gradient-to,{color})", color=color)
        }
    }
}
impl TailwindInstance for TailwindTo {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let color = self.color.get_properties(ctx);
        css_attributes! {
            "--tw-gradient-to" => color
        }
    }
}

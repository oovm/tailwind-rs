mod border_color;
mod border_radius;
mod border_style;
mod border_width;

pub use self::{
    border_color::TailwindBorderColor, border_radius::TailwindRounded, border_style::TailwindBorderStyle,
    border_width::TailwindBorderWidth,
};

pub struct TailwindBorder {
    arbitrary: String
}

impl TailwindBorder {

    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // border
            // [] => TailwindBorderWidth::parse(),

            // https://tailwindcss.com/docs/border-style
            ["solid"] => TailwindBorderStyle::Solid.boxed(),
            ["dashed"] => TailwindBorderStyle::Dashed.boxed(),
            ["dotted"] => TailwindBorderStyle::Dotted.boxed(),
            ["double"] => TailwindBorderStyle::Double.boxed(),
            ["hidden"] => TailwindBorderStyle::Hidden.boxed(),
            ["none"] => TailwindBorderStyle::None.boxed(),
            // https://tailwindcss.com/docs/border-collapse
            ["collapse"] => TailwindBorderCollapse::Collapse.into_instance(),
            ["separate"] => TailwindBorderCollapse::Separate.into_instance(),
            // https://tailwindcss.com/docs/border-color
            ["inherit"] => TailwindBorderCollapse::Collapse.into_instance(),
            ["current"] => TailwindBorderCollapse::Separate.into_instance(),
            ["transparent"] => TailwindBorderCollapse::Separate.into_instance(),
            ["black"] => TailwindBorderCollapse::Separate.into_instance(),
            ["white"] => TailwindBorderCollapse::Separate.into_instance(),

            _ => return syntax_error!("Unknown border instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}


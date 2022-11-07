use super::*;

pub(crate) mod color;
pub(crate) mod line;
pub(crate) mod style;
pub(crate) mod thickness;

#[derive(Debug, Clone)]
pub struct TailwindDecoration {
    arbitrary: TailwindArbitrary,
}

impl TailwindDecoration {
    pub fn adapt(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/text-decoration
            ["line", rest @ ..] => TailwindDecorationLine::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-decoration-style
            [s @ ("solid" | "double" | "dotted" | "dashed" | "wavy")] => TailwindDecorationStyle::from(*s).boxed(),
            ["style", rest @ ..] => TailwindDecorationStyle::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-decoration-thickness
            ["auto"] => TailwindDecorationThickness::from("auto").boxed(),
            ["from", "font"] => TailwindDecorationThickness::from("from-font").boxed(),
            ["thick", rest @ ..] => TailwindDecorationThickness::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["color", rest @ ..] => {
                let color = TailwindColor::parse(rest, arbitrary)?;
                TailwindDecorationColor::from(color).boxed()
            },
            // https://tailwindcss.com/docs/text-decoration-thickness
            [] => TailwindDecoration { arbitrary: TailwindArbitrary::from(arbitrary) }.boxed(),
            [n] => resolve1(n)?,
            _ => TailwindDecorationColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
}

impl Display for TailwindDecoration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.arbitrary.write_class(f, "decoration-")
    }
}

impl TailwindInstance for TailwindDecoration {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "text-decoration" => self.arbitrary.get_properties()
        }
    }
}

fn resolve1(n: &str) -> Result<Box<dyn TailwindInstance>> {
    let a = TailwindArbitrary::from(n);
    if n.starts_with(|c: char| c.is_numeric()) {
        return Ok(resolve1_unit(&a)?.boxed());
    }
    if n.starts_with(|c: char| c == '#') {
        return Ok(resolve1_color(&a)?.boxed());
    }
    Ok(TailwindDecorationColor::from(TailwindColor::Themed(n.to_string(), 0)).boxed())
}

fn resolve1_unit(a: &TailwindArbitrary) -> Result<TailwindDecorationThickness> {
    Ok(TailwindDecorationThickness::from(a.as_integer()?))
}

fn resolve1_color(a: &TailwindArbitrary) -> Result<TailwindDecorationColor> {
    Ok(TailwindDecorationColor::from(a.as_color()?))
}

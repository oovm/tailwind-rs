use super::*;


#[derive(Debug, Copy, Clone)]
enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
    Global(CssBehavior),
}

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindJustifyItems {
    kind: JustifyItems,
}
// justify-items-start	justify-items: start;
// justify-items-end	justify-items: end;
// justify-items-center	justify-items: center;
// justify-items-stretch	justify-items: stretch;
impl Display for JustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for TailwindJustifyItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindJustifyItems {}

impl TailwindJustifyItems {
    /// https://tailwindcss.com/docs/justify-items
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after justify-self");
        let kind = match pattern {
            ["auto"] => JustifySelf::Auto,
            ["start"] => JustifySelf::Start,
            ["end"] => JustifySelf::End,
            ["center"] => JustifySelf::Center,
            ["stretch"] => JustifySelf::Stretch,
            _ => return syntax_error!("Unknown contrast instructions: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}

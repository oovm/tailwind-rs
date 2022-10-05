use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBorderWidth {
    kind: BorderKind,
    width: LengthUnit,
}

#[derive(Copy, Clone, Debug)]
enum BorderKind {
    Border,
    BorderX,
    BorderY,
    BorderT,
    BorderR,
    BorderB,
    BorderL,
}

impl Display for BorderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => write!(f, "border"),
            Self::BorderX => write!(f, "border-x"),
            Self::BorderY => write!(f, "border-y"),
            Self::BorderT => write!(f, "border-t"),
            Self::BorderR => write!(f, "border-r"),
            Self::BorderB => write!(f, "border-b"),
            Self::BorderL => write!(f, "border-l"),
        }
    }
}

impl Display for TailwindBorderWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-[{}]", self.kind, self.width)
    }
}

impl TailwindInstance for TailwindBorderWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let width = self.width.get_properties();
        match self.kind {
            BorderKind::Border => css_attributes! {
                "border-width" => &width
            },
            BorderKind::BorderX => css_attributes! {
                "border-left-width" => &width,
                "border-right-width" => &width,
            },
            BorderKind::BorderY => css_attributes! {
                "border-top-width" => &width,
                "border-bottom-width" => &width,
            },
            BorderKind::BorderT => css_attributes! {
                "border-top-width" => &width,
            },
            BorderKind::BorderR => css_attributes! {
                "border-right-width" => &width,
            },
            BorderKind::BorderB => css_attributes! {
                "border-bottom-width" => &width,
            },
            BorderKind::BorderL => css_attributes! {
                "border-left-width" => &width,
            },
        }
    }
}

impl TailwindBorderWidth {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["t", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderT, arbitrary),
            ["r", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderR, arbitrary),
            ["b", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderB, arbitrary),
            ["l", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderL, arbitrary),
            ["x", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderX, arbitrary),
            ["y", rest @ ..] => Self::parse_inner(rest, BorderKind::BorderY, arbitrary),
            _ => Self::parse_inner(pattern, BorderKind::Border, arbitrary),
        }
    }
    fn parse_inner(pattern: &[&str], kind: BorderKind, arbitrary: &TailwindArbitrary) -> Result<Self> {
        if arbitrary.is_some() {
            Ok(Self { kind, width: arbitrary.as_length_or_fraction()? })
        } else {
            let width = pattern.first().unwrap_or(&"1");
            Ok(Self { kind, width: LengthUnit::px(width.parse()?) })
        }
    }
}

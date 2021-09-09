use super::*;

#[derive(Copy, Clone, Debug)]
enum ColumnKind {
    Auto,
    Columns(u8),
    Length(LengthUnit),
    Global(CssBehavior),
}

#[doc = include_str!("readme.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindColumns {
    kind: ColumnKind,
}
impl Display for ColumnKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Columns(n) => write!(f, "{}", n),
            Self::Length(n) => write!(f, "{}", n),
        }
    }
}

impl Display for TailwindColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "columns-{}", self.kind)
    }
}

impl TailwindInstance for TailwindColumns {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let columns = match self.kind {
            ColumnKind::Auto => "auto".to_string(),
            ColumnKind::Columns(n) => format!("{}", n),
            ColumnKind::Length(n) => format!("{:?}", n),
        };
        css_attributes! {
            "columns" => columns
        }
    }
}
impl ColumnKind {
    #[inline]
    pub fn parse(input: &[&str]) -> Result<Self> {
        let out = match input {
            ["auto"] => Self::Auto,
            ["3xs"] => Self::rem(16),
            ["2xs"] => Self::rem(18),
            ["xs"] => Self::rem(20),
            ["sm"] => Self::rem(24),
            ["md"] => Self::rem(28),
            ["lg"] => Self::rem(32),
            ["xl"] => Self::rem(36),
            ["2xl"] => Self::rem(42),
            ["3xl"] => Self::rem(48),
            ["4xl"] => Self::rem(56),
            ["5xl"] => Self::rem(64),
            ["6xl"] => Self::rem(72),
            ["7xl"] => Self::rem(80),
            [name] => {
                debug_assert!(!name.contains('%'), "forbidden use percent");
                alt((Self::parse_length, Self::parse_columns))(name)?.1
            },
            _ => return syntax_error!("Unknown column instructions: {}", input.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn parse_columns(input: &str) -> IResult<&str, Self> {
        let (rest, i) = parse_integer(input)?;
        Ok((rest, Self::Columns(i)))
    }
    #[inline]
    fn parse_length(input: &str) -> IResult<&str, Self> {
        let (rest, l) = LengthUnit::parse(input)?;
        Ok((rest, Self::Length(l)))
    }
    #[inline(always)]
    fn rem(n: usize) -> ColumnKind {
        Self::Length(LengthUnit::Rem(n as f32))
    }
}

impl TailwindColumns {
    /// https://tailwindcss.com/docs/columns
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after columns");
        Ok(Self { kind: ColumnKind::parse(input)? })
    }
}

use super::*;

#[derive(Debug, Copy, Clone)]
pub(super) enum GridKind {
    Start(GridSize),
    End(GridSize),
    Span(GridSize),
}

#[derive(Debug, Copy, Clone)]
enum GridSize {
    Auto,
    Full,
    Unit(usize),
}

impl Display for GridKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for GridSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridSize::Full => write!(f, ""),
            GridSize::Unit(_) => write!(f, ""),
        }
    }
}

impl GridKind {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after flex-shrink");
        match pattern {
            [] => Ok(Self { shrink: 0 }),
            [n] => Ok(Self { shrink: parse_integer(n)?.1 }),
            _ => syntax_error!("Unknown shrink instructions: {}", pattern.join("-")),
        }
    }
}

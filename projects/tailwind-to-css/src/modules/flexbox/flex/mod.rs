use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub enum TailwindFlex {
    None,
    Inherit,
    Auto { grow: usize, shrink: usize },
    Percent { grow: usize, shrink: usize, basis: usize },
}

impl Display for TailwindFlex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("flex-")?;
        match self {
            Self::None => f.write_str("none"),
            Self::Inherit => {
                todo!()
            },
            Self::Auto { .. } => {
                todo!()
            },
            Self::Percent { .. } => {
                todo!()
            },
        }
    }
}

impl TailwindInstance for TailwindFlex {}

use super::*;

impl Display for TailwindBackgroundColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-{}", self.color)
    }
}

impl TailwindInstance for TailwindBackgroundColor {}

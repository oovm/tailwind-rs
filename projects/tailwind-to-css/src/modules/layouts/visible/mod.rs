use super::*;

/// https://tailwindcss.com/docs/visibility
#[derive(Copy, Clone, Debug)]
pub struct TailwindVisibility {
    kind: Visibility,
}

#[derive(Copy, Clone, Debug)]
enum Visibility {
    Visible,
    Invisible,
}

impl Display for TailwindVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            Visibility::Visible => write!(f, "visible"),
            Visibility::Invisible => write!(f, "invisible"),
        }
    }
}

impl TailwindInstance for TailwindVisibility {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let visibility = match self.kind {
            Visibility::Visible => "visible",
            Visibility::Invisible => "hidden",
        };
        css_attributes! {
         "visibility" =>    visibility
        }
    }
}

impl TailwindVisibility {
    /// `visible`
    pub const Visible: Self = Self { kind: Visibility::Visible };
    /// `invisible`
    pub const Invisible: Self = Self { kind: Visibility::Invisible };
}

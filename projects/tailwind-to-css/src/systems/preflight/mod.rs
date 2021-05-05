use crate::{Result, TailwindInstance};
use std::fmt::Write;

/// https://tailwindcss.com/docs/preflight
#[derive(Clone, Debug)]
pub struct PreflightSystem {
    /// ## Default margins are removed
    /// Preflight removes all of the default margins from elements like headings, blockquotes, paragraphs, etc.
    /// This makes it harder to accidentally rely on margin values applied by the user-agent stylesheet that are not part of your spacing scale.
    pub remove_margins: bool,
    pub unstyle_head: bool,
    pub unstyle_list: bool,
}

impl Default for PreflightSystem {
    fn default() -> Self {
        Self { remove_margins: true, unstyle_head: true, unstyle_list: true }
    }
}

impl PreflightSystem {
    const REMOVE_MARGINS: &'static str = r#"
p, blockquote, hr, dl, dd, h1, h2, h3, h4, h5, h6, figure, pre {
    margin: 0;
}
"#;
    const RESET_HEAD: &'static str = r#"
h1, h2, h3, h4, h5, h6 {
    font-size: inherit;
    font-weight: inherit;
}
"#;
    const RESET_LIST: &'static str = r#"
ol, ul {
    list-style: none;
    margin: 0;
    padding: 0;
}
"#;
}

impl TailwindInstance for PreflightSystem {
    #[track_caller]
    fn id(&self) -> String {
        panic!("can't call id on `PreflightSystem`")
    }

    #[track_caller]
    fn selectors(&self) -> String {
        panic!("can't call selectors on `PreflightSystem`")
    }

    fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        if self.remove_margins {
            f.write_str(Self::REMOVE_MARGINS.trim())?;
            writeln!(f)?;
        }
        if self.unstyle_head {
            f.write_str(Self::RESET_HEAD.trim())?;
            writeln!(f)?;
        }
        if self.unstyle_list {
            f.write_str(Self::RESET_LIST.trim())?;
            writeln!(f)?;
        }
        Ok(())
    }
}

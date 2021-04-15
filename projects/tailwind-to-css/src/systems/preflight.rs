use std::fmt::{Display, Formatter};
use std::io::Write;
use css_style::font::Style;
use crate::CssAttribute;

/// https://tailwindcss.com/docs/preflight
#[derive(Clone, Debug)]
pub struct PreflightSystem {
    /// ## Default margins are removed
    /// Preflight removes all of the default margins from elements like headings, blockquotes, paragraphs, etc.
    /// This makes it harder to accidentally rely on margin values applied by the user-agent stylesheet that are not part of your spacing scale.
    pub remove_margins: bool
}

impl PreflightSystem {
    const REMOVE_MARGINS: &'static str = r#"
p,blockquote,hr,dl,dd,h1,h2,h3,h4,h5,h6,figure,pre{margin:0;}
    "#;
    const RESET_HEAD: &'static str = r#"
h1, h2, h3, h4, h5, h6 {
    font-size: inherit;
    font-weight: inherit;
}
    "#;
    const RESET_LIST: &'static str = r#"\
ol, ul {
    list-style: none;
    margin: 0;
    padding: 0;
}
"#;

    pub fn build(&self) -> &'static str {
        if self.remove_margins { }
    }
}

pub struct CssFormatter<'a> {
    buffer: &'a mut (dyn Write + 'a),
}


impl Display for PreflightSystem  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl CssAttribute for PreflightSystem {
    fn build_css(&self) -> Style {
        todo!()
    }
}
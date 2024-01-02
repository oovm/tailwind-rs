#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]
#![doc(html_favicon_url = "https://upload.wikimedia.org/wikipedia/commons/d/d5/Tailwind_CSS_Logo.svg")]

// Test the README.md code snippets
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub use self::config::CLIConfig;
pub use tailwind_error::{Result, TailwindError};

mod config;
mod processor;
mod support;

pub use tailwind_css::{CssInlineMode, TailwindBuilder};

#[cfg(test)]
mod lib_tests {

    use super::*;

    #[test]
    fn basic() {
        let mut config = CLIConfig::default();
        let mut builder = config.builder();
        config.minify = false;
        builder.preflight.disable = true;

        let input_html = "<div class=\"border-red-500 p-2\"></div>".to_string();

        config.mode = CssInlineMode::None;
        let (html, css) = config.compile_html(&input_html, &mut builder).unwrap();

        assert_eq!(html, input_html);
        assert_eq!(css, ".border-red-500 {\n  border-color: #ef4444;\n}\n\n.p-2 {\n  padding: .5rem;\n}\n");
    }
}

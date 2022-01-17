use std::fmt::{Display, Formatter};

/// <https://tailwindcss.com/docs/preflight>
#[derive(Clone, Debug)]
pub struct PreflightSystem {
    /// disable all preflight
    pub disable: bool,
    /// ## Default margins are removed
    /// Preflight removes all of the default margins from elements like headings, blockquotes, paragraphs, etc.
    /// This makes it harder to accidentally rely on margin values applied by the user-agent stylesheet that are not part of your spacing scale.
    pub remove_margins: bool,
    /// ## Headings are unstyled
    /// All heading elements are completely unstyled by default, and have the same font-size and font-weight as normal text.
    pub unstyle_head: bool,
    /// ## Lists are unstyled
    /// Ordered and unordered lists are unstyled by default, with no bullets/numbers and no margin or padding.
    pub unstyle_list: bool,
    /// ## Images are block-level
    /// Images and other replaced elements (like svg, video, canvas, and others) are display: block by default.
    pub block_level_image: bool,
    /// ## Border styles are reset globally
    /// In order to make it easy to add a border by simply adding the border class, Tailwind overrides the default border styles for all elements with the following rules:
    pub unstyle_border: bool,
    /// ## Buttons have a default outline
    /// To ensure that we provide accessible styles out of the box, we made sure that buttons have a default outline. You can of course override this by applying focus:ring or similar utilities to your buttons.
    pub button_outline: bool,
    /// Custom field for preflight
    pub custom: String,
}

impl Default for PreflightSystem {
    fn default() -> Self {
        Self {
            disable: false,
            remove_margins: true,
            unstyle_head: true,
            unstyle_list: true,
            block_level_image: true,
            unstyle_border: true,
            button_outline: true,
            custom: String::new(),
        }
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
    const IMAGE_BLOCK: &'static str = r#"
img, svg, video, canvas, audio, iframe, embed, object {
  display: block;
  vertical-align: middle;
}
"#;
    // TODO: read theme here
    const RESET_BORDER: &'static str = r#"
*, ::before, ::after {
  border-width: 0;
  border-style: solid;
  border-color: theme('borderColor.DEFAULT', currentColor);
}
"#;
    const BUTTON_OUTLINE: &'static str = r#"
button:focus {
  outline: 1px dotted;
  outline: 5px auto -webkit-focus-ring-color;
}
"#;
}

impl Display for PreflightSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.custom)?;
        // if self.disable { return Ok(()); }
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
        if self.block_level_image {
            f.write_str(Self::IMAGE_BLOCK.trim())?;
            writeln!(f)?;
        }
        if self.unstyle_border {
            f.write_str(Self::RESET_BORDER.trim())?;
            writeln!(f)?;
        }
        if self.button_outline {
            f.write_str(Self::BUTTON_OUTLINE.trim())?;
            writeln!(f)?;
        }
        Ok(())
    }
}

use byte_unit::Byte;
use fs::read_to_string;
use std::{
    env::{current_dir, set_current_dir},
    fs,
    path::Path,
};

use clap::ArgEnum;
use glob::glob;
use tailwind_error::TailwindError;

use tailwind_rs::{CssInlineMode, GlobalConfig, Result, TailwindBuilder};

use crate::TailwindApp;

impl TailwindApp {
    pub fn build_config(&self) -> (GlobalConfig, TailwindBuilder) {
        let mut config = GlobalConfig::default();
        config.css.mode = match self.mode {
            Some(Mode::Inline) => CssInlineMode::Inline,
            Some(Mode::Scope) => CssInlineMode::Scoped,
            Some(Mode::Key) => CssInlineMode::DataKey,
            Some(Mode::Value) => CssInlineMode::DataValue,
            _ => CssInlineMode::None,
        };
        if let Some(s) = self.minify {
            config.css.minify = s;
        }
        config.dry_run = self.dry_run;
        let mut builder = config.builder();
        if let Some(s) = self.obfuscate {
            builder.obfuscate = s;
        }
        self.set_workspace().ok();
        // set_current_dir()
        (config, builder)
    }
    fn set_workspace(&self) -> Result<()> {
        if let Some(s) = &self.workspace {
            set_current_dir(s)?;
        }
        println!("Current workspace: {:?}", current_dir()?);
        Ok(())
    }
}

impl TailwindApp {
    pub fn run(&self, config: &GlobalConfig, builder: &mut TailwindBuilder) -> Result<()> {
        if let Some(c) = &self.command {
            return c.run(config);
        };

        for entry in glob("**/*.html")? {
            let file = entry?;
            let input = read_to_string(&file)?;
            let ext = get_extension(&file).ok_or_else(|| TailwindError::runtime_error("no extension"))?;
            match ext {
                "html" => {
                    let (html, css) = match config.compile_html(&input, builder, &config.css.mode) {
                        Ok(o) => o,
                        Err(e) => {
                            println!("{}", file.display());
                            println!("{}", e);
                            continue;
                        },
                    };
                    if config.dry_run {
                        let html = Byte::from(html.len()).get_appropriate_unit(false);
                        let css = Byte::from(css.len()).get_appropriate_unit(false);
                        println!("dry run on {} success", file.display());
                        println!("HTML size: {}, Css Size: {}", html, css);
                        continue;
                    }
                },
                _ => {
                    todo!("unsupported format {}", ext);
                },
            }
        }
        Ok(())
    }
}

fn get_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Mode {
    Normal,
    Inline,
    Scope,
    Key,
    Value,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}

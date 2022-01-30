use tailwind_css::{CssInlineMode, TailwindBuilder};
use tailwind_rs::CLIConfig;

mod accessibility;
mod arbitrary;
mod background;
mod border;
mod effect;
mod filter;
mod flex;
mod grouped;
mod interactivity;
mod layout;
mod sizing;
mod spacing;
mod svg;
mod table;
mod transform;
mod transition;
mod typography;

fn pre_config() -> (CLIConfig, TailwindBuilder) {
    let mut config = CLIConfig::default();
    let mut builder = config.builder();
    config.minify = false;
    builder.preflight.disable = true;
    (config, builder)
}

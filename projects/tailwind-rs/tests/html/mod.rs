use tailwind_css::{CssInlineMode, TailwindBuilder};
use tailwind_rs::GlobalConfig;

mod accessibility;
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

fn pre_config() -> (GlobalConfig, TailwindBuilder) {
    let mut config = GlobalConfig::default();
    let mut builder = config.builder();
    config.css.minify = false;
    builder.preflight.disable = true;
    (config, builder)
}

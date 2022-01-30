use tailwind_css::{CssInlineMode, TailwindBuilder};
use tailwind_rs::TailwindState;

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

fn pre_config() -> (TailwindState, TailwindBuilder) {
    let mut config = TailwindState::default();
    let mut builder = config.builder();
    config.css.minify = false;
    builder.preflight.disable = true;
    (config, builder)
}

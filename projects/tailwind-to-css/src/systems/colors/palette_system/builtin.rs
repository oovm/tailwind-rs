use super::*;

impl PaletteSystem {
    /// Builtin palettes
    /// <https://tailwindcss.com/docs/customizing-colors>
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.register("slate".to_string(), Palette::slate());
        new.register("gray".to_string(), Palette::gray());
        new.register("zinc".to_string(), Palette::zinc());
        new.register("neutral".to_string(), Palette::neutral());
        new.register("stone".to_string(), Palette::stone());
        new.register("red".to_string(), Palette::red());
        new.register("orange".to_string(), Palette::orange());
        new.register("amber".to_string(), Palette::amber());
        new.register("yellow".to_string(), Palette::yellow());
        new.register("lime".to_string(), Palette::lime());
        new.register("green".to_string(), Palette::green());
        new.register("emerald".to_string(), Palette::emerald());
        new.register("teal".to_string(), Palette::teal());
        new.register("cyan".to_string(), Palette::cyan());
        new.register("sky".to_string(), Palette::sky());
        new.register("blue".to_string(), Palette::blue());
        new.register("indigo".to_string(), Palette::indigo());
        new.register("violet".to_string(), Palette::violet());
        new.register("purple".to_string(), Palette::purple());
        new.register("fuchsia".to_string(), Palette::fuchsia());
        new.register("pink".to_string(), Palette::pink());
        new.register("rose".to_string(), Palette::rose());
        new
    }
}

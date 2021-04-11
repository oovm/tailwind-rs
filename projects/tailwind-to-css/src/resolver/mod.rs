use crate::{BreakPointSystem, FontSystem, PaletteSystem, PreflightSystem};

#[derive(Clone,Debug)]
pub struct TailwindConfig {
    screens: BreakPointSystem,
    colors: PaletteSystem,
    fonts: FontSystem,
    preflight: PreflightSystem,
}

impl Default for TailwindConfig {
    fn default() -> Self {
        Self {
            screens: BreakPointSystem::builtin(),
            colors: PaletteSystem::builtin(),
            fonts: FontSystem::builtin(),
            preflight: PreflightSystem,
        }
    }
}

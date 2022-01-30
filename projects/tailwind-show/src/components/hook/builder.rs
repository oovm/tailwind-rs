use super::*;

pub struct UseTailwindBuilder {}

impl Default for UseTailwindBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl UseTailwindBuilder {
    pub fn use_tailwind<'a>(&self, cx: &'a ScopeState) -> &'a mut UseTailwind {
        cx.use_hook(|_| UseTailwind::new(cx, self))
    }
}

impl UseTailwind {
    fn new(cx: &ScopeState, _: &UseTailwindBuilder) -> Self {
        let mut cfg = CLIConfig::default();
        cfg.builder.preflight.disable = true;
        Self { state: Rc::new(RefCell::new(cfg)), updater: cx.schedule_update() }
    }
}

pub fn use_tailwind(cx: &ScopeState) -> &UseTailwind {
    UseTailwindBuilder::default().use_tailwind(cx)
}

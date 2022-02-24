use std::sync::Arc;

use tailwind_rs::TailwindBuilder;

use super::*;

mod interact;

pub struct UseTailwind {
    config: Rc<RefCell<CLIConfig>>,
    state: Rc<RefCell<TailwindBuilder>>,
    updater: Arc<dyn Fn() + 'static>,
}

pub fn use_tailwind_default(cx: &ScopeState) -> &mut UseTailwind {
    use_tailwind(cx, Default::default())
}

pub fn use_tailwind(cx: &ScopeState, cfg: CLIConfig) -> &mut UseTailwind {
    cx.use_hook(move |_| {
        let init = cfg.builder();
        UseTailwind { config: Rc::new(RefCell::new(cfg)), state: Rc::new(RefCell::new(init)), updater: cx.schedule_update() }
    })
}

impl UseTailwind {
    pub fn get_minify(&self) -> bool {
        self.config.borrow().minify
    }
    pub fn set_minify(&self, minify: bool) {
        self.config.borrow_mut().minify = minify;
        self.needs_update();
    }
    pub fn get_obfuscate(&self) -> bool {
        self.config.borrow().obfuscate
    }
    pub fn set_obfuscate(&self, obfuscate: bool) {
        self.config.borrow_mut().obfuscate = obfuscate;
        self.needs_update();
    }
    pub fn get_preflight(&self) -> bool {
        !self.state.borrow().preflight.disable
    }
    pub fn set_preflight(&self, preflight: bool) {
        self.state.borrow_mut().preflight.disable = !preflight;
        self.needs_update();
    }
    pub fn get_mode(&self) -> CssInlineMode {
        self.config.borrow().mode.clone()
    }
    pub fn set_mode(&self, mode: CssInlineMode) {
        self.config.borrow_mut().mode = mode;
        self.needs_update();
    }
    pub fn needs_update(&self) {
        (self.updater)()
    }
}

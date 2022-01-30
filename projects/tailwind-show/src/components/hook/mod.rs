use super::*;

pub use self::builder::*;

mod builder;

pub struct UseTailwind {
    state: Rc<RefCell<CLIConfig>>,
    updater: Rc<dyn Fn() + 'static>,
}

impl UseTailwind {
    pub fn get_minify(&self) -> bool {
        self.state.borrow().css.minify
    }
    pub fn set_minify(&self, minify: FormEvent) {
        let minify = match minify.value.as_str() {
            "true" => true,
            "false" => false,
            _ => return,
        };
        self.state.borrow_mut().css.minify = minify;
        (self.updater)();
    }
    pub fn get_obfuscate(&self) -> bool {
        self.state.borrow().builder.obfuscate
    }
    pub fn set_obfuscate(&self, obfuscate: FormEvent) {
        let obfuscate = match obfuscate.value.as_str() {
            "true" => true,
            "false" => false,
            _ => return,
        };
        self.state.borrow_mut().builder.obfuscate = obfuscate;
        (self.updater)();
    }
    pub fn get_preflight(&self) -> bool {
        !self.state.borrow().builder.preflight.disable
    }
    pub fn set_preflight(&self, preflight: FormEvent) {
        let preflight = match preflight.value.as_str() {
            "true" => true,
            "false" => false,
            _ => return,
        };
        self.state.borrow_mut().builder.preflight.disable = !preflight;
        (self.updater)();
    }
    pub fn get_mode(&self) -> &'static str {
        match self.state.borrow().css.mode {
            CssInlineMode::None => "m",
            CssInlineMode::Inline => "i",
            CssInlineMode::Scoped => "s",
            CssInlineMode::DataKey => "k",
            CssInlineMode::DataValue => "v",
        }
    }
    pub fn set_mode(&self, preflight: FormEvent) {
        let mode = match preflight.value.as_str() {
            "m" => CssInlineMode::None,
            "i" => CssInlineMode::Inline,
            "s" => CssInlineMode::Scoped,
            "k" => CssInlineMode::DataKey,
            "v" => CssInlineMode::DataValue,
            _ => return,
        };
        self.state.borrow_mut().css.mode = mode;
        (self.updater)();
    }
}

impl UseTailwind {
    pub fn compile(&self, input: &str) -> (LazyNodes, LazyNodes) {
        let mut config = self.state.borrow_mut();
        config.clear();
        match config.compile_html(input) {
            Ok((a, b)) => (
                rsx! {
                    CodeRenderer {
                        code: a,
                        is_error: false,
                    }
                },
                rsx! {
                    CodeRenderer {
                        code: b,
                        is_error: false,
                    }
                },
            ),
            Err(e) => {
                let (a, b) = (e.to_string(), e.to_string());
                (
                    rsx! {
                        CodeRenderer {
                            code: a,
                            is_error: true,
                        }
                    },
                    rsx! {
                        CodeRenderer {
                            code: b,
                            is_error: true,
                        }
                    },
                )
            },
        }
    }
}

use super::*;

impl UseTailwind {
    pub fn MinifyToggle(&self) -> LazyNodes {
        let value = self.get_minify();
        let label = "Minify";
        let click = move |e: FormEvent| {
            let new = match e.value.parse::<bool>() {
                Ok(o) => o,
                Err(_) => return,
            };
            self.set_minify(new);
        };
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "{label}"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{value}",
                    oninput: click
                }
            }
        )
    }
    pub fn ObfuscateToggle(&self) -> LazyNodes {
        let value = self.get_obfuscate();
        let label = "Obfuscate";
        let click = move |e: FormEvent| {
            let new = match e.value.parse::<bool>() {
                Ok(o) => o,
                Err(_) => return,
            };
            self.set_obfuscate(new);
        };
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "{label}"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{value}",
                    oninput: click
                }
            }
        )
    }

    pub fn PreflightToggle(&self) -> LazyNodes {
        let value = self.get_preflight();
        let label = "Preflight";
        let click = move |e: FormEvent| {
            let new = match e.value.parse::<bool>() {
                Ok(o) => o,
                Err(_) => return,
            };
            self.set_preflight(new);
        };
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "{label}"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{value}",
                    oninput: click
                }
            }
        )
    }
    pub fn ModeSelect(&self) -> LazyNodes {
        let value = match self.get_mode() {
            CssInlineMode::None => "m",
            CssInlineMode::Inline => "i",
            CssInlineMode::Scoped => "s",
            CssInlineMode::DataKey => "k",
            CssInlineMode::DataValue => "v",
        };
        let label = "Compile Mode";
        let click = move |e: FormEvent| {
            let new = match e.value.as_str() {
                "i" => CssInlineMode::Inline,
                "s" => CssInlineMode::Scoped,
                "k" => CssInlineMode::DataKey,
                "v" => CssInlineMode::DataValue,
                _ => CssInlineMode::None,
            };
            self.set_mode(new);
        };
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "{label}"
                }
                select {
                    class: "select select-primary w-full max-w-xs",
                    value: "{value}",
                    onchange: click,
                    option {
                        value: "m",
                        "Normal"
                    }
                    option {
                        value: "i",
                        "Inline"
                    }
                    option {
                        value: "s",
                        "Scoped"
                    }
                    option {
                        value: "k",
                        "DataKey"
                    }
                    option {
                        value: "v",
                        "DataValue"
                    }
                }
            }
        )
    }

    pub fn compile(&self, input: &str) -> (LazyNodes, LazyNodes) {
        let config = self.config.borrow_mut();
        let mut builder = self.state.borrow_mut();
        builder.clear();
        match config.compile_html(input, &mut builder) {
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

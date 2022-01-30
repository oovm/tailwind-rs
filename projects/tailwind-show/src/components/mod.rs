#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use dioxus::{events::FormEvent, prelude::*};
use tailwind_rs::{CssInlineMode, TailwindState};

use crate::components::code_render::CodeRenderer;

pub use self::hook::*;

mod code_render;
mod hook;

pub fn Editor(cx: Scope) -> Element {
    let (text, text_set) = use_state(&cx, || String::from(include_str!("../../../tailwind-rs/tests/html/layout/layout.html")));
    let tw = use_tailwind(&cx);
    let is_minify = MinifyToggle(tw);
    let is_obfuscate = ObfuscateToggle(tw);
    let is_preflight = PreflightToggle(tw);
    let which_mode = ModeSelect(tw);
    let (html, css) = tw.compile(text);
    cx.render(rsx!(
        div {
            class: "flex flex-row",
            div {
                class: "form-control flex-1 max-h-screen",
                textarea {
                    class: "textarea h-screen textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "<span class=\"w-1\">tailwind</span>",
                    oninput: move |e| text_set(e.value.to_owned()),
                    value: "{text}",
                }
            }
            div {
                class: "mockup-code flex-1 max-h-screen ml-2 mr-2",
                style: "overflow-y: scroll;",
                id: "html",
                html
            }
            div {
                class: "mockup-code flex-1 max-h-screen",
                style: "overflow-y: scroll;",
                id: "css",
                css
            }
        }
        div {
            class: "form-control",
            is_minify
            is_obfuscate
            is_preflight
            which_mode
            a {
                href: "https://github.com/oovm/tailwind-rs/issues",
                target: "_blank",
                button {
                    class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                    r#type: "button",
                    "Report bug on github"
                }
            }
        }
    ))
}

fn MinifyToggle(tw: &UseTailwind) -> LazyNodes {
    let v = tw.get_minify();
    rsx!(
        label {
            class: "cursor-pointer label",
            span {
                class: "label-text",
                "Minify"
            }
            input {
                r#type: "checkbox",
                class: "toggle",
                checked: "{v}",
                oninput: move |e| tw.set_minify(e)
            }
        }
    )
}

fn ObfuscateToggle(tw: &UseTailwind) -> LazyNodes {
    let v = tw.get_obfuscate();
    rsx!(
        label {
            class: "cursor-pointer label",
            span {
                class: "label-text",
                "Obfuscate"
            }
            input {
                r#type: "checkbox",
                class: "toggle",
                checked: "{v}",
                oninput: move |e| tw.set_obfuscate(e)
            }
        }
    )
}

fn PreflightToggle(tw: &UseTailwind) -> LazyNodes {
    let v = tw.get_preflight();
    rsx!(
        label {
            class: "cursor-pointer label",
            span {
                class: "label-text",
                "Preflight"
            }
            input {
                r#type: "checkbox",
                class: "toggle",
                checked: "{v}",
                oninput: move |e| tw.set_preflight(e)
            }
        }
    )
}

fn ModeSelect(tw: &UseTailwind) -> LazyNodes {
    let v = tw.get_mode();
    rsx!(
        label {
            class: "cursor-pointer label",
            span {
                class: "label-text",
                "Compile Mode"
            }
            select {
                class: "select select-primary w-full max-w-xs",
                value: "{v}",
                onchange: move |e| tw.set_mode(e),
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

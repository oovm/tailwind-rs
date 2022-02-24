#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use dioxus::{events::FormEvent, prelude::*};

use tailwind_rs::{CLIConfig, CssInlineMode};

use crate::components::code_render::CodeRenderer;

pub use self::hook::*;

mod code_render;
mod hook;

pub fn Editor(cx: Scope) -> Element {
    let text = use_state(&cx, || String::from(include_str!("placeholder.html")));
    let tw = use_tailwind_default(&cx);
    let is_minify = tw.MinifyToggle();
    let is_obfuscate = tw.ObfuscateToggle();
    let is_preflight = tw.PreflightToggle();
    let which_mode = tw.ModeSelect();
    let report = GithubIssue("https://github.com/oovm/tailwind-rs/issues");
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
                    oninput: move |e| text.set(e.value.to_owned()),
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
            report
        }
    ))
}

fn GithubIssue(href: &str) -> LazyNodes {
    rsx!(
        a {
            href: "{href}",
            target: "_blank",
            button {
                class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                r#type: "button",
                "Report bug on github"
            }
        }
    )
}

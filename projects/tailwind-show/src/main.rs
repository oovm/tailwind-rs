#![feature(arbitrary_self_types)]
#![allow(non_snake_case)]
#![allow(clippy::mut_from_ref)]
#![allow(clippy::derivable_impls)]

use dioxus::prelude::*;
mod components;

pub use self::components::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(AppWeb)
}

// pub fn main_ssr() {
//     let mut vdom = VirtualDom::new(AppWeb);
//     let _ = vdom.rebuild();
//     println!("{}", dioxus::ssr::render_vdom(&vdom));
// }

pub fn AppWeb(cx: Scope) -> Element {
    cx.render(rsx! {
        Editor {}
    })
}

use super::*;

#[derive(Props, PartialEq)]
pub struct CodeRendererData {
    code: String,
    is_error: bool,
}

pub fn CodeRenderer(cx: Scope<CodeRendererData>) -> Element {
    let class = match cx.props.is_error {
        true => "bg-warning text-neutral",
        false => "",
    };
    let code = cx.props.code.lines().enumerate().map(|(i, t)| {
        rsx!(
            pre {
                class: "{class}",
                "data-prefix": "{i}",
                code {"{t}"}
            }
        )
    });
    cx.render(rsx!(code))
}

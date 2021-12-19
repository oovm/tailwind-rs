use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindContainer {}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "container",)
    }
}
/// .container {
//     width: 100%
// }
//

impl TailwindInstance for TailwindContainer {
    fn inlineable(&self) -> bool {
        false
    }
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "width" => "100%"
        }
    }
    fn additional(&self, _: &TailwindBuilder) -> String {
        r#"
@media(min-width:640px){.container{max-width:640px}}
@media(min-width:720px){.container{max-width:720px}}
@media(min-width:768px){.container{max-width:768px}}
@media(min-width:1024px){.container{max-width:1024px}}
@media(min-width:1280px){.container{max-width:1280px}}
@media(min-width:1536px){.container{max-width:1536px}}
"#
        .to_string()
    }
}

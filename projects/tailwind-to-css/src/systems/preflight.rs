/// https://tailwindcss.com/docs/preflight
pub struct PreflightSystem {}

impl PreflightSystem {
    pub fn remove_margins() -> &'static str {
        r#"\
p,blockquote,
hr,
dl,dd,
h1,h2,h3,h4,h5,h6,
figure,
pre {
  margin: 0;
}"#
    }
    pub fn unstyle_head() -> &'static str {
        r#"\
h1,h2,h3,h4,h5,h6 {
  font-size: inherit;
  font-weight: inherit;
}"#
    }
    pub fn unstyle_list() -> &'static str {
        r#"\
ol,ul {
  list-style: none;
  margin: 0;
  padding: 0;
}"#
    }
}
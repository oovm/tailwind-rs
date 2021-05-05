mod systems;

use tailwind_css::{TailwindBuilder, TailwindObject};

#[test]
fn ready() {
    println!("it works!")
}

pub fn build_target(id: &str, input: &str, inline_target: &str, bundle_target: &str) {
    let mut tailwind = TailwindBuilder::default();
    let inline = TailwindObject { selector: id.to_string(), attributes: tailwind.inline(input) }.to_string();
    tailwind.trace(input);
    let bundle = tailwind.bundle();
    assert_eq!(inline, inline_target);
    assert_eq!(bundle, bundle_target);
}

mod systems;

use tailwind_css::{TailwindBuilder, TailwindObject};

#[test]
fn ready() {
    println!("it works!")
}

pub fn build_target(_: &str, input: &str, inline_target: &str, bundle_target: &str) {
    let mut tailwind = TailwindBuilder::default();
    let inline = tailwind.inline(input);
    tailwind.trace(input);
    let bundle = tailwind.bundle();
    assert_eq!(inline, inline_target);
    assert_eq!(bundle, bundle_target);
}

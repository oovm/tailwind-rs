mod systems;

use tailwind_css::TailwindBuilder;

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

#[test]
fn build() {
    let mut tailwind = TailwindBuilder::default();
    // The compiler will expand directly into the final css property
    // Inline style will not be tracked
    let inline = tailwind.inline("p-auto px-px pt-2 pb-2");
    // The compiler will expand into a `class`, and record the style class used
    tailwind.trace("p-auto px-px pt-2 pb-2");
    // Compile all traced classes into bundle
    let bundle = tailwind.bundle();
    println!("{}", inline);
    println!("{}", bundle);
}

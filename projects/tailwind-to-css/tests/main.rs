use tailwind_css::TailwindBuilder;
#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn build() {
    let mut tailwind = TailwindBuilder::default();
    //
    tailwind.trace("py-2 px-4 bg-green-500");
    println!("{}", tailwind.build().unwrap())
}

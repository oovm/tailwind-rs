pub use macrowind::*;

#[test]
fn test() {
    let (class, style) = tw!("container inset-1 mt-2");
    println!("class: {}", class);
    println!("style: {}", style);
}

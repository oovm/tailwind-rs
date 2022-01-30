Generate css at compile time

```toml
[dev-dependencies]
macrowind = "0.1.*"
```

```rust
pub use macrowind::*;

#[test]
fn test() {
    let (_class, style) = macrowind::tw!("inset-1 mt-2");
    println!("style: {}", style);

    let (class, style) = macrowind::tw!("container");
    println!("class: {}", class);
    println!("style: {}", style);
}
```
# Tailwind to Css



```rust
use tailwind_css::TailwindBuilder;
fn build() {
    let mut tailwind = TailwindBuilder::default();
    // 
    tailwind.trace("py-2 px-4 bg-green-500");
    println!(tailwind.build())
}
```
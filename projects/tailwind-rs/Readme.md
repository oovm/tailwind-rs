# Tailwindcss in Rust

Compile your HTML code into CSS using [Tailwindcss](https://tailwindcss.com/) in Rust.

## Usage

```rust
use tailwind_rs::{CLIConfig, CssInlineMode, TailwindBuilder, Result};

let mut config = CLIConfig::default();
let mut builder = config.builder();
config.minify = false;
builder.preflight.disable = true;

let input_html = "<div class=\"border-red-500 p-2\"></div>".to_string();

config.mode = CssInlineMode::None;
let (html, css) = config.compile_html(&input_html, &mut builder).unwrap();

assert_eq!(html, input_html);
assert_eq!(css, ".border-red-500 {\n  border-color: #ef4444;\n}\n\n.p-2 {\n  padding: .5rem;\n}\n");
```

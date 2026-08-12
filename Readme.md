Tailwind RS
===========

Neutral Tailwind **engine**. Host scanning / CSS file IO are out of core scope.

## Public API

```rust
use tailwind::*;

let out = Engine::new().compile(CompileRequest { /* candidates, theme, … */ });
// out.module — structured Canonical Style Module (not a stylesheet)

// Optional reference CSS lowering (separate crate — not part of compile):
use tailwind_css::serialize_module;
let css = serialize_module(&out.module);
```

Facade crate: **`tailwind`**. CSS text: **`tailwind-css`**.

## Crate layout

| Crate | Role |
|-------|------|
| **[tailwind](projects/tailwind)** | User-facing facade (`tailwind::*`) — Parse → Resolve → Canonicalize |
| **[tailwind-types](projects/tailwind-types)** | Compile contract, Canonical Style Module, miette diagnostics |
| **[tailwind-ast](projects/tailwind-ast)** | Candidate syntax tree |
| **[tailwind-parser](projects/tailwind-parser)** | Built-in lexer + parser (no nom) |
| **[tailwind-resolve](projects/tailwind-resolve)** | ThemeSnapshot, RuleRegistry, VariantRegistry, typed values, canonicalize |
| **[tailwind-css](projects/tailwind-css)** | Reference CSS serializer (consumes module only; not wired into `Engine::compile`) |

**C6 status:** independent CSS serializer exists; **not** production-ready / not Tailwind-complete.
See honest status in `规划设计/vmz/18-TW引擎内核重构路线.md` §12.

Historical old-engine samples (frozen JSON only): **[conformance/legacy](conformance/legacy)**.

Kernel redesign: `规划设计/vmz/18-TW引擎内核重构路线.md`

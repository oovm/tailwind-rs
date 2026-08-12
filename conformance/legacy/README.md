# Legacy engine behavior samples (C0, frozen)

这些样本来自已删除的旧实现 `tailwind-css`（当时 `TailwindBuilder::trace` + `bundle`），**只读冻结**，用于：

1. 记录旧引擎对 spacing / display / color（及少量 variant/important）的实际输出；
2. differential conformance 对照历史缺陷，而不是新引擎的正确性标准。

C4 起旧 crate 已移除，**不再提供再生命令**。以本目录 `samples.json` 为准。

## 注意

- 不是 Canonical Style Module fixture。
- 样本如实记录旧引擎缺陷，例如：
  - variant 前缀在 CSS 中丢失（`hover:bg-red-500` → `.bg-red-500`）
  - `hidden` → `display:hidden`（应为 `none`）
  - `-m-4` 未产生负 margin
  - `!p-4` 解析失败
- 禁止把写 HTML/CSS 文件当作核心 conformance。

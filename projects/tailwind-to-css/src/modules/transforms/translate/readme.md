Utilities for translating elements with transform.


## Standard Pattern

| Pattern             | Properties              |
|:--------------------|:------------------------|
| `translate-[F]`     | `translate([F/4]rem)`   |
| `translate-px`      | `translate(1px)`        |
| `translate-[A]/[B]` | `translate([100×A/B]%)` |
| `translate-full`    | `translate(100%)`       |

Same as `translate-x`, `translate-y`

## Arbitrary Pattern

### `translate-[A]`

```cpp
>> translate-[200px] 
transform: translate(200px)

>> translate-[100px,100px] 
transform: translate(100px,100px);
```

### `translate-x-[A]`

```cpp
>> translate-x-[50%] 
transform: translateX(50%)
```

### `translate-y-[A]`

```cpp
>> translate-y-[50%] 
transform: translateY(50%)
```

## Reference

- [translate](https://tailwindcss.com/docs/translate)
- [translate/cn](https://tailwindcss.cn/docs/translate)
- [transform.translate()](https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/translate())
- [transform.translateX()](https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/translateX())
- [transform.translateY()](https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/translateY())
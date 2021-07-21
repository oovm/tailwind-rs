


```shell
CssAttributes {
    apply: ["text-2"],
    normal: BTreeMap<String, String>,
    transforms: BTreeMap<String, String>,
    scoped: BTreeMap<String, CssAttributes>,
}
```

attribute > apply

```scss
._asdfkw {
    
}
[data-tw-asdfkw] {
    @apply "abc"
    --tw-color: red;
    color: var(--tw-color);
    &:hover {
        
    }
}
```
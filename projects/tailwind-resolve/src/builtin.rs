//! Builtin theme + `register_all_families` — C4 coverage of old `get_instance` families.

use std::collections::BTreeMap;

use tailwind_types::{
    ColorValue, LengthValue, ThemeEntry, ThemeInput, ThemeKey, ThemeValue,
};

use crate::registry::{RuleRegistry, RuleSpec};
use crate::rules::generic::{
    KeywordMapRule, MinMaxKind, StaticRule, ThemeColorRule, ThemeLengthRule,
};
use crate::rules::spacing::{SpacingAxis, SpacingKind, SpacingRule};
use crate::theme::ThemeSnapshot;
use crate::variants::VariantRegistry;

/// Default registries + theme (C4: all major utility families).
pub fn default_rule_registry() -> RuleRegistry {
    let mut r = RuleRegistry::new();
    register_all_families(&mut r);
    r
}

/// Register every migrated family from the old `get_instance` surface.
pub fn register_all_families(registry: &mut RuleRegistry) {
    register_display(registry);
    register_layouts(registry);
    register_flexbox_grid(registry);
    register_spacing(registry);
    register_sizing(registry);
    register_typography(registry);
    register_backgrounds(registry);
    register_borders(registry);
    register_effects(registry);
    register_filters(registry);
    register_tables(registry);
    register_transition(registry);
    register_transforms(registry);
    register_interactivity(registry);
    register_svg(registry);
    register_accessibility(registry);
    registry.arbitrary_property(true);
}

fn register_display(r: &mut RuleRegistry) {
    for name in [
        "block",
        "inline-block",
        "inline",
        "flex",
        "inline-flex",
        "grid",
        "inline-grid",
        "contents",
        "hidden",
        "table",
        "flow-root",
        "list-item",
        "inline-table",
        "table-caption",
        "table-cell",
        "table-column",
        "table-column-group",
        "table-footer-group",
        "table-header-group",
        "table-row-group",
        "table-row",
    ] {
        r.exact(name, RuleSpec::Display);
    }
}

fn register_layouts(r: &mut RuleRegistry) {
    r.functional(
        "float",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "float",
            &[
                ("left", "left"),
                ("right", "right"),
                ("none", "none"),
                ("start", "inline-start"),
                ("end", "inline-end"),
            ],
        )),
    );
    r.functional(
        "clear",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "clear",
            &[
                ("left", "left"),
                ("right", "right"),
                ("both", "both"),
                ("none", "none"),
                ("start", "inline-start"),
                ("end", "inline-end"),
            ],
        )),
    );
    r.functional(
        "overflow",
        RuleSpec::KeywordMap(KeywordMapRule::entries(
            "overflow",
            &[
                ("auto", "overflow", "auto"),
                ("hidden", "overflow", "hidden"),
                ("clip", "overflow", "clip"),
                ("visible", "overflow", "visible"),
                ("scroll", "overflow", "scroll"),
                ("x-auto", "overflow-x", "auto"),
                ("y-auto", "overflow-y", "auto"),
                ("x-hidden", "overflow-x", "hidden"),
                ("y-hidden", "overflow-y", "hidden"),
                ("x-scroll", "overflow-x", "scroll"),
                ("y-scroll", "overflow-y", "scroll"),
                ("x-visible", "overflow-x", "visible"),
                ("y-visible", "overflow-y", "visible"),
                ("x-clip", "overflow-x", "clip"),
                ("y-clip", "overflow-y", "clip"),
            ],
        )),
    );
    r.functional(
        "overscroll",
        RuleSpec::KeywordMap(KeywordMapRule::entries(
            "overscroll-behavior",
            &[
                ("auto", "overscroll-behavior", "auto"),
                ("contain", "overscroll-behavior", "contain"),
                ("none", "overscroll-behavior", "none"),
                ("x-auto", "overscroll-behavior-x", "auto"),
                ("y-auto", "overscroll-behavior-y", "auto"),
                ("x-contain", "overscroll-behavior-x", "contain"),
                ("y-contain", "overscroll-behavior-y", "contain"),
                ("x-none", "overscroll-behavior-x", "none"),
                ("y-none", "overscroll-behavior-y", "none"),
            ],
        )),
    );

    for pos in ["static", "fixed", "absolute", "relative", "sticky"] {
        r.exact(
            pos,
            RuleSpec::Static(StaticRule::pairs(&[("position", pos)])),
        );
    }
    r.functional(
        "position",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "position",
            &[
                ("static", "static"),
                ("fixed", "fixed"),
                ("absolute", "absolute"),
                ("relative", "relative"),
                ("sticky", "sticky"),
            ],
        )),
    );

    let inset_kw = &[
        ("auto", "auto"),
        ("full", "100%"),
        ("px", "1px"),
        ("0", "0px"),
    ];
    r.functional(
        "inset",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["inset"])
                .allow_negative()
                .keywords(inset_kw),
        ),
    );
    r.functional(
        "top",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["top"])
                .allow_negative()
                .keywords(inset_kw),
        ),
    );
    r.functional(
        "right",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["right"])
                .allow_negative()
                .keywords(inset_kw),
        ),
    );
    r.functional(
        "bottom",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["bottom"])
                .allow_negative()
                .keywords(inset_kw),
        ),
    );
    r.functional(
        "left",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["left"])
                .allow_negative()
                .keywords(inset_kw),
        ),
    );

    r.functional("z", RuleSpec::ZIndex);

    r.exact(
        "visible",
        RuleSpec::Static(StaticRule::pairs(&[("visibility", "visible")])),
    );
    r.exact(
        "invisible",
        RuleSpec::Static(StaticRule::pairs(&[("visibility", "hidden")])),
    );
    r.functional(
        "visibility",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "visibility",
            &[("visible", "visible"), ("hidden", "hidden"), ("collapse", "collapse")],
        )),
    );

    r.exact(
        "isolate",
        RuleSpec::Static(StaticRule::pairs(&[("isolation", "isolate")])),
    );
    r.functional(
        "isolation",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "isolation",
            &[("auto", "auto"), ("isolate", "isolate")],
        )),
    );

    r.functional(
        "object",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "object-fit",
            &[
                ("contain", "contain"),
                ("cover", "cover"),
                ("fill", "fill"),
                ("none", "none"),
                ("scale-down", "scale-down"),
            ],
        )),
    );
    // object-position tokens often parse as object-*-* — register common compounds as exact
    for (name, css) in [
        ("object-bottom", "bottom"),
        ("object-center", "center"),
        ("object-left", "left"),
        ("object-left-bottom", "left bottom"),
        ("object-left-top", "left top"),
        ("object-right", "right"),
        ("object-right-bottom", "right bottom"),
        ("object-right-top", "right top"),
        ("object-top", "top"),
    ] {
        r.exact(
            name,
            RuleSpec::Static(StaticRule::pairs(&[("object-position", css)])),
        );
    }

    r.functional(
        "aspect",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "aspect-ratio",
            &[("auto", "auto"), ("square", "1 / 1"), ("video", "16 / 9")],
        )),
    );

    r.exact(
        "box-border",
        RuleSpec::Static(StaticRule::pairs(&[("box-sizing", "border-box")])),
    );
    r.exact(
        "box-content",
        RuleSpec::Static(StaticRule::pairs(&[("box-sizing", "content-box")])),
    );
    r.exact(
        "box-decoration-clone",
        RuleSpec::Static(StaticRule::pairs(&[("box-decoration-break", "clone")])),
    );
    r.exact(
        "box-decoration-slice",
        RuleSpec::Static(StaticRule::pairs(&[("box-decoration-break", "slice")])),
    );

    r.functional(
        "columns",
        RuleSpec::ThemeLength(
            ThemeLengthRule::props(&["columns"], "columns")
                .keywords(&[("auto", "auto")])
                .numeric_suffix(""),
        ),
    );

    r.exact(
        "container",
        RuleSpec::Static(StaticRule::pairs(&[
            ("width", "100%"),
        ])),
    );

    // break-*
    r.functional(
        "break",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "word-break",
            &[
                ("normal", "normal"),
                ("words", "break-word"),
                ("all", "break-all"),
                ("keep", "keep-all"),
            ],
        )),
    );
}

fn register_flexbox_grid(r: &mut RuleRegistry) {
    r.functional("flex", RuleSpec::FlexFunctional);
    r.functional(
        "basis",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["flex-basis"])
                .keywords(&[("auto", "auto"), ("full", "100%")]),
        ),
    );
    r.functional(
        "grow",
        RuleSpec::KeywordMap(
            KeywordMapRule::single("flex-grow", &[("0", "0"), ("", "1")]).with_bare("1"),
        ),
    );
    r.functional(
        "shrink",
        RuleSpec::KeywordMap(
            KeywordMapRule::single("flex-shrink", &[("0", "0"), ("", "1")]).with_bare("1"),
        ),
    );
    r.functional(
        "order",
        RuleSpec::ThemeLength(
            ThemeLengthRule::props(&["order"], "order")
                .allow_negative()
                .keywords(&[
                    ("first", "-9999"),
                    ("last", "9999"),
                    ("none", "0"),
                ])
                .numeric_suffix(""),
        ),
    );

    r.functional(
        "grid",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "grid-template-columns",
            &[
                ("cols-1", "repeat(1, minmax(0, 1fr))"),
                ("cols-2", "repeat(2, minmax(0, 1fr))"),
                ("cols-3", "repeat(3, minmax(0, 1fr))"),
                ("cols-4", "repeat(4, minmax(0, 1fr))"),
                ("cols-5", "repeat(5, minmax(0, 1fr))"),
                ("cols-6", "repeat(6, minmax(0, 1fr))"),
                ("cols-12", "repeat(12, minmax(0, 1fr))"),
                ("cols-none", "none"),
                ("rows-1", "repeat(1, minmax(0, 1fr))"),
                ("rows-2", "repeat(2, minmax(0, 1fr))"),
                ("rows-3", "repeat(3, minmax(0, 1fr))"),
                ("rows-4", "repeat(4, minmax(0, 1fr))"),
                ("rows-6", "repeat(6, minmax(0, 1fr))"),
                ("rows-none", "none"),
                ("flow-row", "row"),
                ("flow-col", "column"),
                ("flow-dense", "dense"),
                ("flow-row-dense", "row dense"),
                ("flow-col-dense", "column dense"),
            ],
        )),
    );
    // Fix grid-flow to use grid-auto-flow — override via exact compounds
    for (name, css) in [
        ("grid-flow-row", "row"),
        ("grid-flow-col", "column"),
        ("grid-flow-dense", "dense"),
        ("grid-flow-row-dense", "row dense"),
        ("grid-flow-col-dense", "column dense"),
    ] {
        r.exact(
            name,
            RuleSpec::Static(StaticRule::pairs(&[("grid-auto-flow", css)])),
        );
    }

    r.functional(
        "col",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "grid-column",
            &[
                ("auto", "auto"),
                ("span-1", "span 1 / span 1"),
                ("span-2", "span 2 / span 2"),
                ("span-3", "span 3 / span 3"),
                ("span-4", "span 4 / span 4"),
                ("span-5", "span 5 / span 5"),
                ("span-6", "span 6 / span 6"),
                ("span-12", "span 12 / span 12"),
                ("span-full", "1 / -1"),
                ("start-1", "1"),
                ("start-2", "2"),
                ("start-auto", "auto"),
                ("end-1", "1"),
                ("end-auto", "auto"),
            ],
        )),
    );
    r.functional(
        "row",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "grid-row",
            &[
                ("auto", "auto"),
                ("span-1", "span 1 / span 1"),
                ("span-2", "span 2 / span 2"),
                ("span-3", "span 3 / span 3"),
                ("span-full", "1 / -1"),
                ("start-1", "1"),
                ("start-auto", "auto"),
                ("end-auto", "auto"),
            ],
        )),
    );
    r.functional(
        "auto",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "grid-auto-columns",
            &[
                ("cols-auto", "auto"),
                ("cols-min", "min-content"),
                ("cols-max", "max-content"),
                ("cols-fr", "minmax(0, 1fr)"),
                ("rows-auto", "auto"),
                ("rows-min", "min-content"),
                ("rows-max", "max-content"),
                ("rows-fr", "minmax(0, 1fr)"),
            ],
        )),
    );

    r.functional("gap", RuleSpec::Gap);

    r.functional(
        "justify",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "justify-content",
            &[
                ("start", "flex-start"),
                ("end", "flex-end"),
                ("center", "center"),
                ("between", "space-between"),
                ("around", "space-around"),
                ("evenly", "space-evenly"),
                ("stretch", "stretch"),
                ("normal", "normal"),
                ("items-start", "start"),
                ("items-end", "end"),
                ("items-center", "center"),
                ("items-stretch", "stretch"),
                ("self-start", "start"),
                ("self-end", "end"),
                ("self-center", "center"),
                ("self-stretch", "stretch"),
                ("self-auto", "auto"),
            ],
        )),
    );
    // justify-items / justify-self via compounds
    for (name, prop, css) in [
        ("justify-items-start", "justify-items", "start"),
        ("justify-items-end", "justify-items", "end"),
        ("justify-items-center", "justify-items", "center"),
        ("justify-items-stretch", "justify-items", "stretch"),
        ("justify-self-auto", "justify-self", "auto"),
        ("justify-self-start", "justify-self", "start"),
        ("justify-self-end", "justify-self", "end"),
        ("justify-self-center", "justify-self", "center"),
        ("justify-self-stretch", "justify-self", "stretch"),
    ] {
        r.exact(name, RuleSpec::Static(StaticRule::pairs(&[(prop, css)])));
    }

    r.functional(
        "content",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "align-content",
            &[
                ("normal", "normal"),
                ("center", "center"),
                ("start", "flex-start"),
                ("end", "flex-end"),
                ("between", "space-between"),
                ("around", "space-around"),
                ("evenly", "space-evenly"),
                ("baseline", "baseline"),
                ("stretch", "stretch"),
            ],
        )),
    );
    r.functional(
        "items",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "align-items",
            &[
                ("start", "flex-start"),
                ("end", "flex-end"),
                ("center", "center"),
                ("baseline", "baseline"),
                ("stretch", "stretch"),
            ],
        )),
    );
    r.functional(
        "self",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "align-self",
            &[
                ("auto", "auto"),
                ("start", "flex-start"),
                ("end", "flex-end"),
                ("center", "center"),
                ("stretch", "stretch"),
                ("baseline", "baseline"),
            ],
        )),
    );
    r.functional(
        "place",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "place-content",
            &[
                ("content-center", "center"),
                ("content-start", "start"),
                ("content-end", "end"),
                ("content-between", "space-between"),
                ("content-around", "space-around"),
                ("content-evenly", "space-evenly"),
                ("content-baseline", "baseline"),
                ("content-stretch", "stretch"),
                ("items-start", "start"),
                ("items-end", "end"),
                ("items-center", "center"),
                ("items-baseline", "baseline"),
                ("items-stretch", "stretch"),
                ("self-auto", "auto"),
                ("self-start", "start"),
                ("self-end", "end"),
                ("self-center", "center"),
                ("self-stretch", "stretch"),
            ],
        )),
    );
    for (name, prop, css) in [
        ("place-items-start", "place-items", "start"),
        ("place-items-end", "place-items", "end"),
        ("place-items-center", "place-items", "center"),
        ("place-items-baseline", "place-items", "baseline"),
        ("place-items-stretch", "place-items", "stretch"),
        ("place-self-auto", "place-self", "auto"),
        ("place-self-start", "place-self", "start"),
        ("place-self-end", "place-self", "end"),
        ("place-self-center", "place-self", "center"),
        ("place-self-stretch", "place-self", "stretch"),
        ("place-content-center", "place-content", "center"),
        ("place-content-start", "place-content", "start"),
        ("place-content-end", "place-content", "end"),
        ("place-content-between", "place-content", "space-between"),
        ("place-content-around", "place-content", "space-around"),
        ("place-content-evenly", "place-content", "space-evenly"),
        ("place-content-baseline", "place-content", "baseline"),
        ("place-content-stretch", "place-content", "stretch"),
    ] {
        r.exact(name, RuleSpec::Static(StaticRule::pairs(&[(prop, css)])));
    }
}

fn register_spacing(r: &mut RuleRegistry) {
    let pad = |axis| RuleSpec::Spacing(SpacingRule::new(SpacingKind::Padding, axis));
    let mar = |axis| RuleSpec::Spacing(SpacingRule::new(SpacingKind::Margin, axis));

    r.functional("p", pad(SpacingAxis::All));
    r.functional("px", pad(SpacingAxis::X));
    r.functional("py", pad(SpacingAxis::Y));
    r.functional("pt", pad(SpacingAxis::Top));
    r.functional("pr", pad(SpacingAxis::Right));
    r.functional("pb", pad(SpacingAxis::Bottom));
    r.functional("pl", pad(SpacingAxis::Left));

    r.functional("m", mar(SpacingAxis::All));
    r.functional("mx", mar(SpacingAxis::X));
    r.functional("my", mar(SpacingAxis::Y));
    r.functional("mt", mar(SpacingAxis::Top));
    r.functional("mr", mar(SpacingAxis::Right));
    r.functional("mb", mar(SpacingAxis::Bottom));
    r.functional("ml", mar(SpacingAxis::Left));
    // space-x/y omitted: requires child combinator rules beyond DeclarationSet.
}

fn size_keywords_w() -> &'static [(&'static str, &'static str)] {
    &[
        ("auto", "auto"),
        ("full", "100%"),
        ("screen", "100vw"),
        ("svw", "100svw"),
        ("lvw", "100lvw"),
        ("dvw", "100dvw"),
        ("min", "min-content"),
        ("max", "max-content"),
        ("fit", "fit-content"),
    ]
}

fn size_keywords_h() -> &'static [(&'static str, &'static str)] {
    &[
        ("auto", "auto"),
        ("full", "100%"),
        ("screen", "100vh"),
        ("svh", "100svh"),
        ("lvh", "100lvh"),
        ("dvh", "100dvh"),
        ("min", "min-content"),
        ("max", "max-content"),
        ("fit", "fit-content"),
    ]
}

fn register_sizing(r: &mut RuleRegistry) {
    r.functional(
        "w",
        RuleSpec::ThemeLength(ThemeLengthRule::spacing(&["width"]).keywords(size_keywords_w())),
    );
    r.functional(
        "h",
        RuleSpec::ThemeLength(ThemeLengthRule::spacing(&["height"]).keywords(size_keywords_h())),
    );
    r.functional(
        "size",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["width", "height"]).keywords(size_keywords_w()),
        ),
    );
    r.functional("min", RuleSpec::MinMaxSize(MinMaxKind::Min));
    r.functional("max", RuleSpec::MinMaxSize(MinMaxKind::Max));
}

fn register_typography(r: &mut RuleRegistry) {
    r.functional("text", RuleSpec::Text);

    r.functional(
        "font",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "font-weight",
            &[
                ("thin", "100"),
                ("extralight", "200"),
                ("light", "300"),
                ("normal", "400"),
                ("medium", "500"),
                ("semibold", "600"),
                ("bold", "700"),
                ("extrabold", "800"),
                ("black", "900"),
            ],
        )),
    );
    // font-sans etc. as exact family
    for (name, css) in [
        (
            "font-sans",
            "ui-sans-serif, system-ui, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\"",
        ),
        ("font-serif", "ui-serif, Georgia, Cambria, \"Times New Roman\", Times, serif"),
        ("font-mono", "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace"),
    ] {
        r.exact(
            name,
            RuleSpec::Static(StaticRule::pairs(&[("font-family", css)])),
        );
    }

    r.exact(
        "antialiased",
        RuleSpec::Static(StaticRule::pairs(&[
            ("-webkit-font-smoothing", "antialiased"),
            ("-moz-osx-font-smoothing", "grayscale"),
        ])),
    );
    r.exact(
        "subpixel-antialiased",
        RuleSpec::Static(StaticRule::pairs(&[
            ("-webkit-font-smoothing", "auto"),
            ("-moz-osx-font-smoothing", "auto"),
        ])),
    );
    r.exact(
        "italic",
        RuleSpec::Static(StaticRule::pairs(&[("font-style", "italic")])),
    );
    r.exact(
        "not-italic",
        RuleSpec::Static(StaticRule::pairs(&[("font-style", "normal")])),
    );

    r.functional(
        "tracking",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "letter-spacing",
            &[
                ("tighter", "-0.05em"),
                ("tight", "-0.025em"),
                ("normal", "0em"),
                ("wide", "0.025em"),
                ("wider", "0.05em"),
                ("widest", "0.1em"),
            ],
        )),
    );
    r.functional(
        "leading",
        RuleSpec::ThemeLength(
            ThemeLengthRule::props(&["line-height"], "lineHeight").keywords(&[
                ("none", "1"),
                ("tight", "1.25"),
                ("snug", "1.375"),
                ("normal", "1.5"),
                ("relaxed", "1.625"),
                ("loose", "2"),
            ]),
        ),
    );

    r.functional(
        "list",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "list-style-type",
            &[
                ("none", "none"),
                ("disc", "disc"),
                ("decimal", "decimal"),
                ("image-none", "none"),
            ],
        )),
    );
    r.exact(
        "list-inside",
        RuleSpec::Static(StaticRule::pairs(&[("list-style-position", "inside")])),
    );
    r.exact(
        "list-outside",
        RuleSpec::Static(StaticRule::pairs(&[("list-style-position", "outside")])),
    );

    r.exact(
        "underline",
        RuleSpec::Static(StaticRule::pairs(&[("text-decoration-line", "underline")])),
    );
    r.exact(
        "overline",
        RuleSpec::Static(StaticRule::pairs(&[("text-decoration-line", "overline")])),
    );
    r.exact(
        "line-through",
        RuleSpec::Static(StaticRule::pairs(&[("text-decoration-line", "line-through")])),
    );
    r.exact(
        "no-underline",
        RuleSpec::Static(StaticRule::pairs(&[("text-decoration-line", "none")])),
    );

    r.functional(
        "decoration",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "text-decoration-style",
            &[
                ("solid", "solid"),
                ("double", "double"),
                ("dotted", "dotted"),
                ("dashed", "dashed"),
                ("wavy", "wavy"),
            ],
        )),
    );

    r.exact(
        "uppercase",
        RuleSpec::Static(StaticRule::pairs(&[("text-transform", "uppercase")])),
    );
    r.exact(
        "lowercase",
        RuleSpec::Static(StaticRule::pairs(&[("text-transform", "lowercase")])),
    );
    r.exact(
        "capitalize",
        RuleSpec::Static(StaticRule::pairs(&[("text-transform", "capitalize")])),
    );
    r.exact(
        "normal-case",
        RuleSpec::Static(StaticRule::pairs(&[("text-transform", "none")])),
    );

    r.exact(
        "truncate",
        RuleSpec::Static(StaticRule::pairs(&[
            ("overflow", "hidden"),
            ("text-overflow", "ellipsis"),
            ("white-space", "nowrap"),
        ])),
    );
    r.exact(
        "text-ellipsis",
        RuleSpec::Static(StaticRule::pairs(&[("text-overflow", "ellipsis")])),
    );
    r.exact(
        "text-clip",
        RuleSpec::Static(StaticRule::pairs(&[("text-overflow", "clip")])),
    );

    r.functional(
        "indent",
        RuleSpec::ThemeLength(ThemeLengthRule::spacing(&["text-indent"]).allow_negative()),
    );
    r.functional(
        "align",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "vertical-align",
            &[
                ("baseline", "baseline"),
                ("top", "top"),
                ("middle", "middle"),
                ("bottom", "bottom"),
                ("text-top", "text-top"),
                ("text-bottom", "text-bottom"),
                ("sub", "sub"),
                ("super", "super"),
            ],
        )),
    );
    r.functional(
        "whitespace",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "white-space",
            &[
                ("normal", "normal"),
                ("nowrap", "nowrap"),
                ("pre", "pre"),
                ("pre-line", "pre-line"),
                ("pre-wrap", "pre-wrap"),
                ("break-spaces", "break-spaces"),
            ],
        )),
    );

    // text-align via exact (text-left would conflict with Text color/size — use exact compounds)
    for (name, css) in [
        ("text-left", "left"),
        ("text-center", "center"),
        ("text-right", "right"),
        ("text-justify", "justify"),
        ("text-start", "start"),
        ("text-end", "end"),
    ] {
        r.exact(
            name,
            RuleSpec::Static(StaticRule::pairs(&[("text-align", css)])),
        );
    }
}

fn register_backgrounds(r: &mut RuleRegistry) {
    r.functional("bg", RuleSpec::BackgroundColor);
    r.functional("from", RuleSpec::ThemeColor(ThemeColorRule::new("--tw-gradient-from")));
    r.functional("via", RuleSpec::ThemeColor(ThemeColorRule::new("--tw-gradient-via")));
    r.functional("to", RuleSpec::ThemeColor(ThemeColorRule::new("--tw-gradient-to")));
}

fn register_borders(r: &mut RuleRegistry) {
    r.functional("rounded", RuleSpec::Rounded);
    r.functional("border", RuleSpec::Border);
    r.functional(
        "divide",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "border-style",
            &[
                ("solid", "solid"),
                ("dashed", "dashed"),
                ("dotted", "dotted"),
                ("double", "double"),
                ("none", "none"),
                ("x", "solid"),
                ("y", "solid"),
            ],
        )),
    );
    r.functional(
        "outline",
        RuleSpec::KeywordMap(
            KeywordMapRule::single(
                "outline-style",
                &[
                    ("none", "none"),
                    ("solid", "solid"),
                    ("dashed", "dashed"),
                    ("dotted", "dotted"),
                    ("double", "double"),
                    ("hidden", "hidden"),
                ],
            )
            .with_bare("solid"),
        ),
    );
    r.functional(
        "ring",
        RuleSpec::ThemeLength(
            ThemeLengthRule::props(&["--tw-ring-offset-shadow", "box-shadow"], "ringWidth")
                .bare("0 0 0 3px var(--tw-ring-color, rgb(59 130 246 / 0.5))")
                .keywords(&[
                    ("0", "0 0 0 0px var(--tw-ring-color)"),
                    ("1", "0 0 0 1px var(--tw-ring-color)"),
                    ("2", "0 0 0 2px var(--tw-ring-color)"),
                    ("4", "0 0 0 4px var(--tw-ring-color)"),
                    ("8", "0 0 0 8px var(--tw-ring-color)"),
                    ("inset", "inset 0 0 0 3px var(--tw-ring-color)"),
                ]),
        ),
    );
}

fn register_effects(r: &mut RuleRegistry) {
    r.functional("shadow", RuleSpec::Shadow);
    r.functional("opacity", RuleSpec::Opacity);
    r.exact(
        "mix-blend-normal",
        RuleSpec::Static(StaticRule::pairs(&[("mix-blend-mode", "normal")])),
    );
    r.exact(
        "mix-blend-multiply",
        RuleSpec::Static(StaticRule::pairs(&[("mix-blend-mode", "multiply")])),
    );
    r.exact(
        "mix-blend-screen",
        RuleSpec::Static(StaticRule::pairs(&[("mix-blend-mode", "screen")])),
    );
    r.exact(
        "mix-blend-overlay",
        RuleSpec::Static(StaticRule::pairs(&[("mix-blend-mode", "overlay")])),
    );
    r.functional(
        "mix",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "mix-blend-mode",
            &[
                ("blend-normal", "normal"),
                ("blend-multiply", "multiply"),
                ("blend-screen", "screen"),
                ("blend-overlay", "overlay"),
                ("blend-darken", "darken"),
                ("blend-lighten", "lighten"),
                ("blend-difference", "difference"),
            ],
        )),
    );
}

const BLUR_KW: &[(&str, &str)] = &[
    ("none", "0"),
    ("sm", "4px"),
    ("", "8px"),
    ("md", "12px"),
    ("lg", "16px"),
    ("xl", "24px"),
    ("2xl", "40px"),
    ("3xl", "64px"),
];

const BRIGHTNESS_KW: &[(&str, &str)] = &[
    ("0", "0"),
    ("50", ".5"),
    ("75", ".75"),
    ("90", ".9"),
    ("95", ".95"),
    ("100", "1"),
    ("105", "1.05"),
    ("110", "1.1"),
    ("125", "1.25"),
    ("150", "1.5"),
    ("200", "2"),
];
const CONTRAST_KW: &[(&str, &str)] = &[
    ("0", "0"),
    ("50", ".5"),
    ("75", ".75"),
    ("100", "1"),
    ("125", "1.25"),
    ("150", "1.5"),
    ("200", "2"),
];
const GRAYSCALE_KW: &[(&str, &str)] = &[("0", "0")];
const INVERT_KW: &[(&str, &str)] = &[("0", "0")];
const SATURATE_KW: &[(&str, &str)] = &[
    ("0", "0"),
    ("50", ".5"),
    ("100", "1"),
    ("150", "1.5"),
    ("200", "2"),
];
const SEPIA_KW: &[(&str, &str)] = &[("0", "0")];

fn register_filters(r: &mut RuleRegistry) {
    r.functional(
        "blur",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "blur",
            keywords: BLUR_KW,
            bare: "8px",
        },
    );
    r.functional(
        "brightness",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "brightness",
            keywords: BRIGHTNESS_KW,
            bare: "1",
        },
    );
    r.functional(
        "contrast",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "contrast",
            keywords: CONTRAST_KW,
            bare: "1",
        },
    );
    r.functional(
        "grayscale",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "grayscale",
            keywords: GRAYSCALE_KW,
            bare: "100%",
        },
    );
    r.functional(
        "invert",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "invert",
            keywords: INVERT_KW,
            bare: "100%",
        },
    );
    r.functional(
        "saturate",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "saturate",
            keywords: SATURATE_KW,
            bare: "1",
        },
    );
    r.functional(
        "sepia",
        RuleSpec::FilterFn {
            property: "filter",
            fn_name: "sepia",
            keywords: SEPIA_KW,
            bare: "100%",
        },
    );
    r.exact(
        "drop-shadow-sm",
        RuleSpec::Static(StaticRule::pairs(&[(
            "filter",
            "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
        )])),
    );
    r.exact(
        "drop-shadow",
        RuleSpec::Static(StaticRule::pairs(&[(
            "filter",
            "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
        )])),
    );
    r.functional(
        "backdrop",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "backdrop-filter",
            &[
                ("blur-sm", "blur(4px)"),
                ("blur", "blur(8px)"),
                ("blur-md", "blur(12px)"),
                ("grayscale", "grayscale(100%)"),
                ("invert", "invert(100%)"),
                ("opacity-50", "opacity(0.5)"),
            ],
        )),
    );
}

fn register_tables(r: &mut RuleRegistry) {
    r.functional(
        "table",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "table-layout",
            &[("auto", "auto"), ("fixed", "fixed")],
        )),
    );
    r.exact(
        "border-collapse",
        RuleSpec::Static(StaticRule::pairs(&[("border-collapse", "collapse")])),
    );
    r.exact(
        "border-separate",
        RuleSpec::Static(StaticRule::pairs(&[("border-collapse", "separate")])),
    );
}

fn register_transition(r: &mut RuleRegistry) {
    r.functional(
        "transition",
        RuleSpec::KeywordMap(
            KeywordMapRule::single(
                "transition-property",
                &[
                    ("none", "none"),
                    ("all", "all"),
                    (
                        "",
                        "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
                    ),
                    ("colors", "color, background-color, border-color, text-decoration-color, fill, stroke"),
                    ("opacity", "opacity"),
                    ("shadow", "box-shadow"),
                    ("transform", "transform"),
                ],
            )
            .with_bare(
                "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
            ),
        ),
    );
    r.functional("duration", RuleSpec::TimeMs("transition-duration"));
    r.functional("delay", RuleSpec::TimeMs("transition-delay"));
    r.functional(
        "ease",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "transition-timing-function",
            &[
                ("linear", "linear"),
                ("in", "cubic-bezier(0.4, 0, 1, 1)"),
                ("out", "cubic-bezier(0, 0, 0.2, 1)"),
                ("in-out", "cubic-bezier(0.4, 0, 0.2, 1)"),
            ],
        )),
    );
    r.functional(
        "animate",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "animation",
            &[
                ("none", "none"),
                ("spin", "spin 1s linear infinite"),
                ("ping", "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"),
                ("pulse", "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"),
                ("bounce", "bounce 1s infinite"),
            ],
        )),
    );
}

fn register_transforms(r: &mut RuleRegistry) {
    r.functional("scale", RuleSpec::Scale);
    r.functional("rotate", RuleSpec::Rotate);
    r.functional(
        "translate",
        RuleSpec::ThemeLength(
            ThemeLengthRule::spacing(&["translate"])
                .allow_negative()
                .keywords(&[("full", "100%"), ("1/2", "50%")]),
        ),
    );
    r.functional(
        "skew",
        RuleSpec::ThemeLength(
            ThemeLengthRule::props(&["transform"], "skew")
                .allow_negative()
                .numeric_suffix("deg")
                .wrap("skewX({v})"),
        ),
    );
    r.functional(
        "origin",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "transform-origin",
            &[
                ("center", "center"),
                ("top", "top"),
                ("top-right", "top right"),
                ("right", "right"),
                ("bottom-right", "bottom right"),
                ("bottom", "bottom"),
                ("bottom-left", "bottom left"),
                ("left", "left"),
                ("top-left", "top left"),
            ],
        )),
    );
}

fn register_interactivity(r: &mut RuleRegistry) {
    r.functional("accent", RuleSpec::ThemeColor(ThemeColorRule::new("accent-color")));
    r.functional(
        "appearance",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "appearance",
            &[("none", "none"), ("auto", "auto")],
        )),
    );
    r.functional(
        "cursor",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "cursor",
            &[
                ("auto", "auto"),
                ("default", "default"),
                ("pointer", "pointer"),
                ("wait", "wait"),
                ("text", "text"),
                ("move", "move"),
                ("help", "help"),
                ("not-allowed", "not-allowed"),
                ("none", "none"),
                ("context-menu", "context-menu"),
                ("progress", "progress"),
                ("cell", "cell"),
                ("crosshair", "crosshair"),
                ("grab", "grab"),
                ("grabbing", "grabbing"),
                ("zoom-in", "zoom-in"),
                ("zoom-out", "zoom-out"),
            ],
        )),
    );
    r.functional("caret", RuleSpec::ThemeColor(ThemeColorRule::new("caret-color")));
    r.exact(
        "pointer-events-none",
        RuleSpec::Static(StaticRule::pairs(&[("pointer-events", "none")])),
    );
    r.exact(
        "pointer-events-auto",
        RuleSpec::Static(StaticRule::pairs(&[("pointer-events", "auto")])),
    );
    r.functional(
        "resize",
        RuleSpec::KeywordMap(
            KeywordMapRule::single(
                "resize",
                &[
                    ("none", "none"),
                    ("y", "vertical"),
                    ("x", "horizontal"),
                    ("", "both"),
                ],
            )
            .with_bare("both"),
        ),
    );
    r.functional(
        "scroll",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "scroll-behavior",
            &[("auto", "auto"), ("smooth", "smooth")],
        )),
    );
    r.functional(
        "snap",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "scroll-snap-type",
            &[
                ("start", "start"),
                ("end", "end"),
                ("center", "center"),
                ("align-none", "none"),
                ("normal", "normal"),
                ("always", "always"),
                ("none", "none"),
                ("x", "x var(--tw-scroll-snap-strictness)"),
                ("y", "y var(--tw-scroll-snap-strictness)"),
                ("both", "both var(--tw-scroll-snap-strictness)"),
                ("mandatory", "mandatory"),
                ("proximity", "proximity"),
            ],
        )),
    );
    r.functional(
        "touch",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "touch-action",
            &[
                ("auto", "auto"),
                ("none", "none"),
                ("pan-x", "pan-x"),
                ("pan-y", "pan-y"),
                ("manipulation", "manipulation"),
            ],
        )),
    );
    r.functional(
        "select",
        RuleSpec::KeywordMap(KeywordMapRule::single(
            "user-select",
            &[
                ("none", "none"),
                ("text", "text"),
                ("all", "all"),
                ("auto", "auto"),
            ],
        )),
    );
    r.exact(
        "will-change-auto",
        RuleSpec::Static(StaticRule::pairs(&[("will-change", "auto")])),
    );
    r.exact(
        "will-change-scroll",
        RuleSpec::Static(StaticRule::pairs(&[("will-change", "scroll-position")])),
    );
    r.exact(
        "will-change-contents",
        RuleSpec::Static(StaticRule::pairs(&[("will-change", "contents")])),
    );
    r.exact(
        "will-change-transform",
        RuleSpec::Static(StaticRule::pairs(&[("will-change", "transform")])),
    );
}

fn register_svg(r: &mut RuleRegistry) {
    r.functional("fill", RuleSpec::ThemeColor(ThemeColorRule::new("fill")));
    r.functional("stroke", RuleSpec::ThemeColor(ThemeColorRule::new("stroke")));
}

fn register_accessibility(r: &mut RuleRegistry) {
    r.exact(
        "sr-only",
        RuleSpec::Static(StaticRule::pairs(&[
            ("position", "absolute"),
            ("width", "1px"),
            ("height", "1px"),
            ("padding", "0"),
            ("margin", "-1px"),
            ("overflow", "hidden"),
            ("clip", "rect(0, 0, 0, 0)"),
            ("white-space", "nowrap"),
            ("border-width", "0"),
        ])),
    );
    r.exact(
        "not-sr-only",
        RuleSpec::Static(StaticRule::pairs(&[
            ("position", "static"),
            ("width", "auto"),
            ("height", "auto"),
            ("padding", "0"),
            ("margin", "0"),
            ("overflow", "visible"),
            ("clip", "auto"),
            ("white-space", "normal"),
        ])),
    );
}

pub fn default_variant_registry() -> VariantRegistry {
    let mut v = VariantRegistry::new();
    v.register_selector("hover", ":hover", 100, true);
    v.register_selector("focus", ":focus", 110, true);
    v.register_selector("active", ":active", 120, true);
    v.register_selector("dark", ".dark &", 50, false);
    v.register_selector("before", "::before", 200, false);
    v.register_selector("after", "::after", 210, false);
    v
}

pub fn default_breakpoints() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("sm".into(), "(min-width: 640px)".into()),
        ("md".into(), "(min-width: 768px)".into()),
        ("lg".into(), "(min-width: 1024px)".into()),
        ("xl".into(), "(min-width: 1280px)".into()),
        ("2xl".into(), "(min-width: 1536px)".into()),
    ])
}

fn push_length(entries: &mut Vec<ThemeEntry>, ns: &str, key: &str, css: &str) {
    entries.push(ThemeEntry {
        key: ThemeKey::from_path([ns, key]),
        value: ThemeValue::Length(LengthValue {
            css: css.into(),
        }),
    });
}

fn push_keyword(entries: &mut Vec<ThemeEntry>, ns: &str, key: &str, css: &str) {
    entries.push(ThemeEntry {
        key: ThemeKey::from_path([ns, key]),
        value: ThemeValue::Keyword(css.into()),
    });
}

fn push_number(entries: &mut Vec<ThemeEntry>, ns: &str, key: &str, css: &str) {
    entries.push(ThemeEntry {
        key: ThemeKey::from_path([ns, key]),
        value: ThemeValue::Number(css.into()),
    });
}

fn push_color(entries: &mut Vec<ThemeEntry>, token: &str, css: &str) {
    let mut path = vec!["colors".to_string()];
    path.extend(token.split('-').map(str::to_string));
    entries.push(ThemeEntry {
        key: ThemeKey { path },
        value: ThemeValue::Color(ColorValue { css: css.into() }),
    });
}

pub fn builtin_theme_input() -> ThemeInput {
    let mut entries = Vec::new();

    let spacing = [
        ("0", "0px"),
        ("px", "1px"),
        ("0.5", "0.125rem"),
        ("1", "0.25rem"),
        ("1.5", "0.375rem"),
        ("2", "0.5rem"),
        ("2.5", "0.625rem"),
        ("3", "0.75rem"),
        ("3.5", "0.875rem"),
        ("4", "1rem"),
        ("5", "1.25rem"),
        ("6", "1.5rem"),
        ("7", "1.75rem"),
        ("8", "2rem"),
        ("9", "2.25rem"),
        ("10", "2.5rem"),
        ("11", "2.75rem"),
        ("12", "3rem"),
        ("14", "3.5rem"),
        ("16", "4rem"),
        ("20", "5rem"),
        ("24", "6rem"),
        ("28", "7rem"),
        ("32", "8rem"),
        ("36", "9rem"),
        ("40", "10rem"),
        ("44", "11rem"),
        ("48", "12rem"),
        ("52", "13rem"),
        ("56", "14rem"),
        ("60", "15rem"),
        ("64", "16rem"),
        ("72", "18rem"),
        ("80", "20rem"),
        ("96", "24rem"),
    ];
    for (k, v) in spacing {
        push_length(&mut entries, "spacing", k, v);
    }
    push_keyword(&mut entries, "spacing", "auto", "auto");

    // Core palette shades 50–900
    let palettes: &[(&str, &[(&str, &str)])] = &[
        (
            "slate",
            &[
                ("50", "#f8fafc"),
                ("100", "#f1f5f9"),
                ("200", "#e2e8f0"),
                ("300", "#cbd5e1"),
                ("400", "#94a3b8"),
                ("500", "#64748b"),
                ("600", "#475569"),
                ("700", "#334155"),
                ("800", "#1e293b"),
                ("900", "#0f172a"),
            ],
        ),
        (
            "gray",
            &[
                ("50", "#f9fafb"),
                ("100", "#f3f4f6"),
                ("200", "#e5e7eb"),
                ("300", "#d1d5db"),
                ("400", "#9ca3af"),
                ("500", "#6b7280"),
                ("600", "#4b5563"),
                ("700", "#374151"),
                ("800", "#1f2937"),
                ("900", "#111827"),
            ],
        ),
        (
            "red",
            &[
                ("50", "#fef2f2"),
                ("100", "#fee2e2"),
                ("200", "#fecaca"),
                ("300", "#fca5a5"),
                ("400", "#f87171"),
                ("500", "#ef4444"),
                ("600", "#dc2626"),
                ("700", "#b91c1c"),
                ("800", "#991b1b"),
                ("900", "#7f1d1d"),
            ],
        ),
        (
            "orange",
            &[
                ("50", "#fff7ed"),
                ("100", "#ffedd5"),
                ("200", "#fed7aa"),
                ("300", "#fdba74"),
                ("400", "#fb923c"),
                ("500", "#f97316"),
                ("600", "#ea580c"),
                ("700", "#c2410c"),
                ("800", "#9a3412"),
                ("900", "#7c2d12"),
            ],
        ),
        (
            "amber",
            &[
                ("50", "#fffbeb"),
                ("100", "#fef3c7"),
                ("200", "#fde68a"),
                ("300", "#fcd34d"),
                ("400", "#fbbf24"),
                ("500", "#f59e0b"),
                ("600", "#d97706"),
                ("700", "#b45309"),
                ("800", "#92400e"),
                ("900", "#78350f"),
            ],
        ),
        (
            "yellow",
            &[
                ("50", "#fefce8"),
                ("100", "#fef9c3"),
                ("200", "#fef08a"),
                ("300", "#fde047"),
                ("400", "#facc15"),
                ("500", "#eab308"),
                ("600", "#ca8a04"),
                ("700", "#a16207"),
                ("800", "#854d0e"),
                ("900", "#713f12"),
            ],
        ),
        (
            "green",
            &[
                ("50", "#f0fdf4"),
                ("100", "#dcfce7"),
                ("200", "#bbf7d0"),
                ("300", "#86efac"),
                ("400", "#4ade80"),
                ("500", "#22c55e"),
                ("600", "#16a34a"),
                ("700", "#15803d"),
                ("800", "#166534"),
                ("900", "#14532d"),
            ],
        ),
        (
            "emerald",
            &[
                ("50", "#ecfdf5"),
                ("100", "#d1fae5"),
                ("200", "#a7f3d0"),
                ("300", "#6ee7b7"),
                ("400", "#34d399"),
                ("500", "#10b981"),
                ("600", "#059669"),
                ("700", "#047857"),
                ("800", "#065f46"),
                ("900", "#064e3b"),
            ],
        ),
        (
            "teal",
            &[
                ("50", "#f0fdfa"),
                ("100", "#ccfbf1"),
                ("200", "#99f6e4"),
                ("300", "#5eead4"),
                ("400", "#2dd4bf"),
                ("500", "#14b8a6"),
                ("600", "#0d9488"),
                ("700", "#0f766e"),
                ("800", "#115e59"),
                ("900", "#134e4a"),
            ],
        ),
        (
            "cyan",
            &[
                ("50", "#ecfeff"),
                ("100", "#cffafe"),
                ("200", "#a5f3fc"),
                ("300", "#67e8f9"),
                ("400", "#22d3ee"),
                ("500", "#06b6d4"),
                ("600", "#0891b2"),
                ("700", "#0e7490"),
                ("800", "#155e75"),
                ("900", "#164e63"),
            ],
        ),
        (
            "sky",
            &[
                ("50", "#f0f9ff"),
                ("100", "#e0f2fe"),
                ("200", "#bae6fd"),
                ("300", "#7dd3fc"),
                ("400", "#38bdf8"),
                ("500", "#0ea5e9"),
                ("600", "#0284c7"),
                ("700", "#0369a1"),
                ("800", "#075985"),
                ("900", "#0c4a6e"),
            ],
        ),
        (
            "blue",
            &[
                ("50", "#eff6ff"),
                ("100", "#dbeafe"),
                ("200", "#bfdbfe"),
                ("300", "#93c5fd"),
                ("400", "#60a5fa"),
                ("500", "#3b82f6"),
                ("600", "#2563eb"),
                ("700", "#1d4ed8"),
                ("800", "#1e40af"),
                ("900", "#1e3a8a"),
            ],
        ),
        (
            "indigo",
            &[
                ("50", "#eef2ff"),
                ("100", "#e0e7ff"),
                ("200", "#c7d2fe"),
                ("300", "#a5b4fc"),
                ("400", "#818cf8"),
                ("500", "#6366f1"),
                ("600", "#4f46e5"),
                ("700", "#4338ca"),
                ("800", "#3730a3"),
                ("900", "#312e81"),
            ],
        ),
        (
            "violet",
            &[
                ("50", "#f5f3ff"),
                ("100", "#ede9fe"),
                ("200", "#ddd6fe"),
                ("300", "#c4b5fd"),
                ("400", "#a78bfa"),
                ("500", "#8b5cf6"),
                ("600", "#7c3aed"),
                ("700", "#6d28d9"),
                ("800", "#5b21b6"),
                ("900", "#4c1d95"),
            ],
        ),
        (
            "purple",
            &[
                ("50", "#faf5ff"),
                ("100", "#f3e8ff"),
                ("200", "#e9d5ff"),
                ("300", "#d8b4fe"),
                ("400", "#c084fc"),
                ("500", "#a855f7"),
                ("600", "#9333ea"),
                ("700", "#7e22ce"),
                ("800", "#6b21a8"),
                ("900", "#581c87"),
            ],
        ),
        (
            "fuchsia",
            &[
                ("50", "#fdf4ff"),
                ("100", "#fae8ff"),
                ("200", "#f5d0fe"),
                ("300", "#f0abfc"),
                ("400", "#e879f9"),
                ("500", "#d946ef"),
                ("600", "#c026d3"),
                ("700", "#a21caf"),
                ("800", "#86198f"),
                ("900", "#701a75"),
            ],
        ),
        (
            "pink",
            &[
                ("50", "#fdf2f8"),
                ("100", "#fce7f3"),
                ("200", "#fbcfe8"),
                ("300", "#f9a8d4"),
                ("400", "#f472b6"),
                ("500", "#ec4899"),
                ("600", "#db2777"),
                ("700", "#be185d"),
                ("800", "#9d174d"),
                ("900", "#831843"),
            ],
        ),
        (
            "rose",
            &[
                ("50", "#fff1f2"),
                ("100", "#ffe4e6"),
                ("200", "#fecdd3"),
                ("300", "#fda4af"),
                ("400", "#fb7185"),
                ("500", "#f43f5e"),
                ("600", "#e11d48"),
                ("700", "#be123c"),
                ("800", "#9f1239"),
                ("900", "#881337"),
            ],
        ),
    ];
    push_color(&mut entries, "black", "#000000");
    push_color(&mut entries, "white", "#ffffff");
    push_color(&mut entries, "transparent", "transparent");
    push_color(&mut entries, "current", "currentColor");
    for (name, shades) in palettes {
        for (shade, hex) in *shades {
            push_color(&mut entries, &format!("{name}-{shade}"), hex);
        }
    }

    for (k, v) in [
        ("xs", "0.75rem"),
        ("sm", "0.875rem"),
        ("base", "1rem"),
        ("lg", "1.125rem"),
        ("xl", "1.25rem"),
        ("2xl", "1.5rem"),
        ("3xl", "1.875rem"),
        ("4xl", "2.25rem"),
        ("5xl", "3rem"),
        ("6xl", "3.75rem"),
        ("7xl", "4.5rem"),
        ("8xl", "6rem"),
        ("9xl", "8rem"),
    ] {
        push_length(&mut entries, "fontSize", k, v);
    }

    for (k, v) in [
        ("none", "0px"),
        ("sm", "0.125rem"),
        ("", "0.25rem"),
        ("md", "0.375rem"),
        ("lg", "0.5rem"),
        ("xl", "0.75rem"),
        ("2xl", "1rem"),
        ("3xl", "1.5rem"),
        ("full", "9999px"),
    ] {
        if k.is_empty() {
            push_length(&mut entries, "borderRadius", "DEFAULT", v);
        } else {
            push_length(&mut entries, "borderRadius", k, v);
        }
    }

    for (k, v) in [("0", "0px"), ("2", "2px"), ("4", "4px"), ("8", "8px"), ("DEFAULT", "1px")] {
        push_length(&mut entries, "borderWidth", k, v);
    }

    for (k, v) in [
        ("0", "0"),
        ("5", "0.05"),
        ("10", "0.1"),
        ("20", "0.2"),
        ("25", "0.25"),
        ("30", "0.3"),
        ("40", "0.4"),
        ("50", "0.5"),
        ("60", "0.6"),
        ("70", "0.7"),
        ("75", "0.75"),
        ("80", "0.8"),
        ("90", "0.9"),
        ("95", "0.95"),
        ("100", "1"),
    ] {
        push_number(&mut entries, "opacity", k, v);
    }

    for (k, v) in [
        ("0", "0"),
        ("10", "10"),
        ("20", "20"),
        ("30", "30"),
        ("40", "40"),
        ("50", "50"),
        ("auto", "auto"),
    ] {
        if k == "auto" {
            push_keyword(&mut entries, "zIndex", k, v);
        } else {
            push_number(&mut entries, "zIndex", k, v);
        }
    }

    for (k, v) in [("3", "0.75"), ("4", "1"), ("5", "1.25"), ("6", "1.5"), ("7", "1.75"), ("8", "2"), ("9", "2.25"), ("10", "2.5")] {
        push_number(&mut entries, "lineHeight", k, v);
    }

    ThemeInput { entries }
}

pub fn default_theme(overrides: ThemeInput) -> Result<ThemeSnapshot, Vec<tailwind_types::Diagnostic>> {
    ThemeSnapshot::normalize(builtin_theme_input(), overrides, default_breakpoints())
}

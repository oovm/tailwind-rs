(* ::Package:: *)

(* ::Package:: *)


SetDirectory[NotebookDirectory[]];
predefined = Import["css-color-names.json", {"JSON"}];
match[color_ -> hex_] := match[
    color,
    RGBColor[hex][[1]] * 255 // Round,
    RGBColor[hex][[2]] * 255 // Round,
    RGBColor[hex][[3]] * 255 // Round
];
match[color_, r_, g_, b_] := TemplateApply[
    "_ if const_eq!(name, \"`1`\") => Color { r: `2`, g: `3`, b: `4`, a: 0.0 },",
    {color, r, g, b}
];




codegen = TemplateApply[
    "use super::*;

impl Color {
    #[doc = include_str!(\"predefined.md\")]
    #[track_caller]
    pub const fn predefined(name: &str) -> Color {
        match () {
            `1`,
            _ => panic!(\"unknown color code\"),
        }
    }
}
", {StringRiffle[match /@ predefined, "\n            "]}
];
Export["predefined.rs", codegen, "Text"]

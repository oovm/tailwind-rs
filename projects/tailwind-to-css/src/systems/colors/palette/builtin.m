(* ::Package:: *)

getColorMap[color_] := <|
    "name" -> First@Cases[color, XMLElement["div", {"class" -> "text-sm font-semibold text-slate-900 dark:text-slate-200"}, {name_}] :> name, Infinity],
    "map" -> Cases[color, {XMLElement["div", {__}, {n_}], XMLElement["div", {__}, {a_}]} :> {n, a}, Infinity]
|>;
asMarkdown = StringRiffle[TemplateApply["<span style=\"color:`2`\">`1`</span>", #]& /@ #map, ",\n    ///"]&;
asRust = StringRiffle[TemplateApply["colors.insert(`1`,Srgb::from_str(\"`2`\").unwrap())", #]& /@ #map, ";\n        "]&;
buildFunction = TemplateApply["\
    /// ## `Name`
    ///`asMarkdown`
    pub fn `name`() -> Self {
        let mut colors = BTreeMap::default();
		`asRust`;
        Self { gradient: true, key_points: colors }
    }
",
    <|
        "Name" -> ToUpperCase@#name,
        "name" -> ToLowerCase@#name,
        "asMarkdown" -> asMarkdown[#],
        "asRust" -> asRust[#]
    |>
]&;


colors = Import["https://tailwindcss.com/docs/customizing-colors", {"XHTML", "XMLObject"}];
colors = Cases[colors, XMLElement["div", {"class" -> "grid grid-cols-1 gap-8"}, xml___] :> xml, Infinity] // Flatten;
colors = getColorMap /@ colors;
codegen = StringJoin[Flatten[{
    "\
use super::*;
/// Builtin colors
/// https://tailwindcss.com/docs/customizing-colors
impl Palette {
",
    buildFunction /@ colors,
    "}"
}], "\n"];


reg = TemplateApply["new.register(\"`1`\".to_string(), Palette::`1`());", {ToLowerCase@#name}]& /@ colors;
StringRiffle[reg, "\n"] // CopyToClipboard;


SetDirectory@NotebookDirectory[];
Export["builtin.rs", codegen, "Text"]

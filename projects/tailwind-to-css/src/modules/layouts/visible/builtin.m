(* ::Package:: *)

values = "
/* Keyword values */
visibility: visible;
visibility: hidden;
visibility: collapse;

/* Global values */
visibility: inherit;
visibility: initial;
visibility: revert;
visibility: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

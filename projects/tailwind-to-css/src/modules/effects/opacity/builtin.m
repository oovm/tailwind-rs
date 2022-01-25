(* ::Package:: *)

values = "
opacity: 0.9
opacity: 90%

/* Global values */
opacity: inherit;
opacity: initial;
opacity: revert;
opacity: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

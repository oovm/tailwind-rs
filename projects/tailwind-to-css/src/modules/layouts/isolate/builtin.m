(* ::Package:: *)

values = "
/* Keyword values */
isolation: auto;
isolation: isolate;

/* Global values */
isolation: inherit;
isolation: initial;
isolation: revert;
isolation: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"let set = BTreeSet::from_iter(vec![\"" <> StringRiffle[all, "\",\""] <> "\"]);" // CopyToClipboard

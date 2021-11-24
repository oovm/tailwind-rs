(* ::Package:: *)

values = "
/* Global values */
aspect-ratio: inherit;
aspect-ratio: initial;
aspect-ratio: revert;
aspect-ratio: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"let set = BTreeSet::from_iter(vec![\"" <> StringRiffle[all, "\",\""] <> "\"]);" // CopyToClipboard

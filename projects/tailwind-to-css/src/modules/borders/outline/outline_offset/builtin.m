(* ::Package:: *)

values = "
/* <length> values */
outline-offset: 3.0px;
outline-offset: 0.2em;

/* Global values */
outline-offset: inherit;
outline-offset: initial;
outline-offset: revert;
outline-offset: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"let set = BTreeSet::from_iter(vec![\"" <> StringRiffle[all, "\",\""] <> "\"]);" // CopyToClipboard

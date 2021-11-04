(* ::Package:: *)

values = "
/* Keyword values */
scroll-snap-align: none;
scroll-snap-align: start end; /* when two values set first is block, second inline */
scroll-snap-align: center;

/* Global values */
scroll-snap-align: inherit;
scroll-snap-align: initial;
scroll-snap-align: revert;
scroll-snap-align: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"let set = BTreeSet::from_iter(vec![\"" <> StringRiffle[all, "\",\""] <> "\"]);" // CopyToClipboard

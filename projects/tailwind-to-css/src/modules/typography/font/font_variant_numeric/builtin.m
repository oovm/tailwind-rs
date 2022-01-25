(* ::Package:: *)

pointer = "
font-variant-numeric: normal;
font-variant-numeric: ordinal;
font-variant-numeric: slashed-zero;
font-variant-numeric: lining-nums;         /* <numeric-figure-values> */
font-variant-numeric: oldstyle-nums;       /* <numeric-figure-values> */
font-variant-numeric: proportional-nums;   /* <numeric-spacing-values> */
font-variant-numeric: tabular-nums;        /* <numeric-spacing-values> */
font-variant-numeric: diagonal-fractions;  /* <numeric-fraction-values> */
font-variant-numeric: stacked-fractions;   /* <numeric-fraction-values> */
font-variant-numeric: oldstyle-nums stacked-fractions;

/* Global values */
font-variant-numeric: inherit;
font-variant-numeric: initial;
font-variant-numeric: unset;
";

all = Sort@StringCases[
    pointer,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

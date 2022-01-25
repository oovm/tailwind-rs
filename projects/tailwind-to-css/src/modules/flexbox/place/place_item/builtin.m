(* ::Package:: *)

values = "
/* Keyword values */
place-items: center;
place-items: normal start;

/* Positional alignment */
place-items: center normal;
place-items: start legacy;
place-items: end normal;
place-items: self-start legacy;
place-items: self-end normal;
place-items: flex-start legacy;
place-items: flex-end normal;
place-items: left legacy;
place-items: right normal;

/* Baseline alignment */
place-items: baseline normal;
place-items: first baseline legacy;
place-items: last baseline normal;
place-items: stretch legacy;

/* Global values */
place-items: inherit;
place-items: initial;
place-items: revert;
place-items: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

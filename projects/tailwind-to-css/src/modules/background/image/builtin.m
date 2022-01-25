(* ::Package:: *)

values = "
/* Keyword values */
background-image: none;

background-image:
  linear-gradient(to bottom, rgba(255,255,0,0.5), rgba(0,0,255,0.5)),
  url('catfront.png');

/* Global values */
background-image: inherit;
background-image: initial;
background-image: revert;
background-image: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

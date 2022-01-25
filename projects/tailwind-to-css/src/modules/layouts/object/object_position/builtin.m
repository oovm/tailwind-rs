(* ::Package:: *)

values = "
/* <position> values */
object-position: center top;
object-position: 100px 50px;

/* Global values */
object-position: inherit;
object-position: initial;
object-position: revert;
object-position: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

(* ::Package:: *)

values = "
/* CSS Basic User Interface Module Level 4 values */
appearance: none;
appearance: auto;
appearance: menulist-button;
appearance: textfield;

/* \"Compat-auto\" values, which have the same effect as 'auto' */
appearance: button;
appearance: searchfield;
appearance: textarea;
appearance: push-button;
appearance: slider-horizontal;
appearance: checkbox;
appearance: radio;
appearance: square-button;
appearance: menulist;
appearance: listbox;
appearance: meter;
appearance: progress-bar;

/* Partial list of available values in Gecko */
-moz-appearance: scrollbarbutton-up;
-moz-appearance: button-bevel;

/* Partial list of available values in WebKit/Blink (as well as Gecko and Edge) */
-webkit-appearance: media-mute-button;
-webkit-appearance: caret;

/* Global values */
appearance: inherit;
appearance: initial;
appearance: revert;
appearance: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

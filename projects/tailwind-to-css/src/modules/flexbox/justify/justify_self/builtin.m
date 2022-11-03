(* ::Package:: *)

name = "justify-self";
values = "
/* Basic keywords */
justify-self: auto;
justify-self: normal;
justify-self: stretch;

/* Positional alignment */
justify-self: center;     /* Pack item around the center */
justify-self: start;      /* Pack item from the start */
justify-self: end;        /* Pack item from the end */
justify-self: flex-start; /* Equivalent to 'start'. Note that justify-self is ignored in Flexbox layouts. */
justify-self: flex-end;   /* Equivalent to 'end'. Note that justify-self is ignored in Flexbox layouts. */
justify-self: self-start;
justify-self: self-end;
justify-self: left;       /* Pack item from the left */
justify-self: right;      /* Pack item from the right */

/* Baseline alignment */
justify-self: baseline;
justify-self: first baseline;
justify-self: last baseline;

/* Overflow alignment (for positional alignment only) */
justify-self: safe center;
justify-self: unsafe center;

/* Global values */
justify-self: inherit;
justify-self: initial;
justify-self: revert;
justify-self: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard


TemplateApply[
"
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/`name`#syntax>
    pub const KEYWORDS: KeywordMap = &[
        \"auto\",
        \"baseline\",
        \"center\",
        \"end\",
        \"flex-end\",
        \"flex-start\",
        \"inherit\",
        \"initial\",
        \"left\",
        \"normal\",
        \"revert\",
        \"right\",
        \"self-end\",
        \"self-start\",
        \"start\",
        \"stretch\",
        \"unset\",
    ];
", <|"name"->name|>
]




(* ::Package:: *)

values = "
/* Keywords that cannot be combined with other values */
content: normal;
content: none;

/* <image> values */
content: url(\"http://www.example.com/test.png\");
content: linear-gradient(#e66465, #9198e5);
content: image-set(\"image1x.png\" 1x, \"image2x.png\" 2x);

/* alt text for generated content, added in the Level 3 specification */
content: url(\"http://www.example.com/test.png\") / \"This is the alt text\";

/* <string> value */
content: \"prefix\";

/* <counter> values, optionally with <list-style-type> */
content: counter(chapter_counter);
content: counter(chapter_counter, upper-roman);
content: counters(section_counter, \".\");
content: counters(section_counter, \".\", decimal-leading-zero);

/* attr() value linked to the HTML attribute value */
content: attr(value string);

/* Language- and position-dependent keywords */
content: open-quote;
content: close-quote;
content: no-open-quote;
content: no-close-quote;

/* Except for normal and none, several values can be used simultaneously */
content: open-quote counter(chapter_counter);

/* Global values */
content: inherit;
content: initial;
content: revert;
content: unset;
";

all = Sort@StringCases[
    values,
    RegularExpression[":\\s*([a-zA-Z0-9-]+);"] :> "$1"
];
"[\"" <> StringRiffle[all, "\",\""] <> "\"].contains(&mode)" // CopyToClipboard

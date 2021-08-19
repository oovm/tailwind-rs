use crate::CssAttribute;

// Background Repeat
// Utilities for controlling the repetition of an element's background image.
//
// ​
// Quick reference
// Class
// Properties
// bg-repeat	background-repeat: repeat;
// bg-no-repeat	background-repeat: no-repeat;
// bg-repeat-x	background-repeat: repeat-x;
// bg-repeat-y	background-repeat: repeat-y;
// bg-repeat-round	background-repeat: round;
// bg-repeat-space	background-repeat: space;
#[derive(Copy, Clone, Debug)]
enum BackgroundRepeat {}

// https://tailwindcss.com/docs/background-origin
#[derive(Copy, Clone, Debug)]
pub struct TailwindBackgroundRepeat {
    kind: BackgroundOrigin,
}

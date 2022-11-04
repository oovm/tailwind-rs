use crate::test_expand;

// not-hover:sm:text-red-[200/50]
pub const INPUT1: &str = r#"
text

text!
text(red bold)!
text-(red-bold!)!
text(blue(bold!)!)!

first-line:text
last-child:text
not-first-line:not-last-child:text

text-[red]
"#;

pub const OUTPUT1: &str = r#"
text
text!
text-red!
text-bold!
text-red-bold!
text-blue-bold!
first-line::text
last-child:text
not-first-line::not-last-child:text
text-[red]
"#;

#[test]
fn test_group() {
    test_expand(INPUT1, OUTPUT1, 8);
}

// not-hover:sm:text-red-[200/50]
pub const INPUT2: &str = r#"
grid-cols-[[linename],1fr,auto]
w-[{}]
w-[{{}}]
w-[()]
w-[(())]
w-[[]]
w-[[[]]]
w-[{[}]]
w-[[{]}]
w-[{(})]
w-[({)}]
w-[([)]]
w-[[(])]
w-[)(]
w-[}{]
w-[)()]
w-[}{}]
w-['][]']
w-[')()']
w-['}{}']
"#;

pub const OUTPUT2: &str = r#"
grid-cols-[[linename],1fr,auto]
w-[{}]
w-[{{}}]
w-[()]
w-[(())]
w-[[]]
w-[[[]]]
w-[{[}]]
w-[[{]}]
w-[{(})]
w-[({)}]
w-[([)]]
w-[[(])]
w-[)(]
w-[}{]
w-[)()]
w-[}{}]
w-[][]]
w-[)()]
w-[}{}]
"#;

#[test]
fn test_arbitrary() {
    test_expand(INPUT2, OUTPUT2, 8);
}

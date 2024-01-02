use crate::AstGroupItem::{Grouped, Styled};

use super::*;

#[test]
fn test_reference() {
    let input = AstReference::parse("&").unwrap().1;
    let output = AstReference {};
    assert_eq!(input, output);
}

fn assert_arbitrary(input: &str, output: &str) {
    let input = AstArbitrary::parse(input).unwrap().1;
    let output = AstArbitrary { arbitrary: output };
    assert_eq!(input, output);
}

#[test]
fn test_arbitrary() {
    assert_arbitrary("-[#FFF]", "#FFF");
    assert_arbitrary("-[\\]]", "]");
    assert_arbitrary("-[']']", "']'");
    assert_arbitrary("-[[line-name],1fr,auto]", "[line-name],1fr,auto");
}

#[test]
#[should_panic]
fn test_arbitrary_bad1() {
    AstArbitrary::parse("-[]").unwrap();
}

#[test]
fn test_variant() {
    let input = ASTVariant::parse("not-hover::").unwrap().1;
    let output = ASTVariant { not: true, pseudo: true, names: vec!["hover"] };
    assert_eq!(input, output);
    let input = ASTVariant::parse("sm:").unwrap().1;
    let output = ASTVariant { not: false, pseudo: false, names: vec!["sm"] };
    assert_eq!(input, output);
}

#[test]
fn test_style() {
    let input = AstStyle::parse("full").unwrap().1;
    let output = AstStyle {
        negative: false,
        variants: vec![],
        elements: vec!["full"],
        arbitrary: None,
        ..Default::default()
    };
    assert_eq!(input, output);
    let input = AstStyle::parse("-top-1").unwrap().1;
    let output = AstStyle {
        negative: true,
        variants: vec![],
        elements: vec!["top", "1"],
        arbitrary: None,
        ..Default::default()
    };
    assert_eq!(input, output);
    let input = AstStyle::parse("not-hover:sm:text-red-[200/50]").unwrap().1;
    let output = AstStyle {
        negative: false,
        variants: vec![
            ASTVariant { not: true, pseudo: false, names: vec!["hover"] },
            ASTVariant { not: false, pseudo: false, names: vec!["sm"] },
        ],
        elements: vec!["text", "red"],
        arbitrary: Some("200/50"),
        ..Default::default()
    };
    assert_eq!(input, output);
}

// #[test]
// #[should_panic]
// fn test_style_bad1() {
//     AstStyle::parse(":a-[]").unwrap();
// }

#[test]
fn test_group() {
    let input = AstGroup::parse("w(full sm:auto)").unwrap().1;
    let output = AstGroup {
        head: AstStyle {
            negative: false,
            variants: vec![],
            elements: vec!["w"],
            arbitrary: None,
            ..Default::default()
        },
        children: vec![
            Styled(AstStyle {
                negative: false,
                variants: vec![],
                elements: vec!["full"],
                arbitrary: None,
                ..Default::default()
            }),
            Styled(AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["sm"] }],
                elements: vec!["auto"],
                arbitrary: None,
                ..Default::default()
            }),
        ],
        ..Default::default()
    };
    assert_eq!(input, output);
    let input = AstGroup::parse("rotate(-3 hover:6 md:(3 hover:-6))").unwrap().1;
    let output = AstGroup {
        head: AstStyle {
            negative: false,
            variants: vec![],
            elements: vec!["rotate"],
            arbitrary: None,
            ..Default::default()
        },
        children: vec![
            Styled(AstStyle {
                negative: true,
                variants: vec![],
                elements: vec!["3"],
                arbitrary: None,
                ..Default::default()
            }),
            Styled(AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["hover"] }],
                elements: vec!["6"],
                arbitrary: None,
                ..Default::default()
            }),
            Grouped(AstGroup {
                head: AstStyle {
                    negative: false,
                    variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["md"] }],
                    elements: vec![],
                    arbitrary: None,
                    ..Default::default()
                },
                children: vec![
                    Styled(AstStyle {
                        negative: false,
                        variants: vec![],
                        elements: vec!["3"],
                        arbitrary: None,
                        ..Default::default()
                    }),
                    Styled(AstStyle {
                        negative: true,
                        variants: vec![ASTVariant {
                            not: false,
                            pseudo: false,
                            names: vec!["hover"],
                        }],
                        elements: vec!["6"],
                        arbitrary: None,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
        ],
        ..Default::default()
    };
    assert_eq!(input, output);
    let input = AstGroup::parse("bg-blue-500(hover:& focus:& active:&)").unwrap().1;
    let output = AstGroup {
        head: AstStyle {
            negative: false,
            variants: vec![],
            elements: vec!["bg", "blue", "500"],
            arbitrary: None,
            ..Default::default()
        },
        children: vec![
            Styled(AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["hover"] }],
                elements: vec!["&"],
                arbitrary: None,
                ..Default::default()
            }),
            Styled(AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["focus"] }],
                elements: vec!["&"],
                arbitrary: None,
                ..Default::default()
            }),
            Styled(AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["active"] }],
                elements: vec!["&"],
                arbitrary: None,
                ..Default::default()
            }),
        ],
        ..Default::default()
    };
    assert_eq!(input, output);
}

#[track_caller]
fn check_expand(input: &str, target: &str) {
    let styles = parse_tailwind(input).unwrap();
    let v: Vec<_> = styles.iter().map(|s| s.to_string()).collect();
    let output = v.join(" ");
    assert_eq!(target, output)
}

#[test]
fn test_expand() {
    check_expand("not-hover:sm:text-red-200", "not-hover:sm:text-red-200");
    check_expand("w(full sm:auto)", "w-full sm:w-auto");
    check_expand("w(1/2 sm:1/3 lg:1/6) p-2", "w-1/2 sm:w-1/3 lg:w-1/6 p-2");
    check_expand(
        "rotate(-3 hover:6 md:(3 hover:-6))",
        "-rotate-3 hover:rotate-6 md:rotate-3 md:hover:-rotate-6",
    );
    check_expand(
        "ring(& pink-700 offset(4 pink-200))",
        "ring ring-pink-700 ring-offset-4 ring-offset-pink-200",
    );
    check_expand(
        r#"
              bg-red-500 shadow-xs
              sm:(bg-red-600 shadow-sm)
              md:(
                bg-red-700 shadow(md)
              )
              lg:(
                bg-red-800 
                shadow(xl)
              )
        "#,
        "bg-red-500 shadow-xs sm:bg-red-600 sm:shadow-sm md:bg-red-700 md:shadow-md lg:bg-red-800 lg:shadow-xl",
    );
}

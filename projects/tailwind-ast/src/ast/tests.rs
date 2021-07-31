use super::*;

#[test]
fn test_reference() {
    let input = AstReference::parse("&").unwrap().1;
    let output = AstReference {};
    assert_eq!(input, output);
}

#[test]
fn test_arbitrary() {
    let input = AstArbitrary::parse("-[#FFF]").unwrap().1;
    let output = AstArbitrary { arbitrary: "#FFF" };
    assert_eq!(input, output);
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
        //
        negative: false,
        variants: vec![],
        elements: vec!["full"],
        arbitrary: None,
    };
    assert_eq!(input, output);
    let input = AstStyle::parse("-top-1").unwrap().1;
    let output = AstStyle {
        //
        negative: true,
        variants: vec![],
        elements: vec!["top", "1"],
        arbitrary: None,
    };
    assert_eq!(input, output);
    let input = AstStyle::parse("not-hover:sm:text-red-[200/50]").unwrap().1;
    let output = AstStyle {
        //
        negative: false,
        variants: vec![
            ASTVariant { not: true, pseudo: false, names: vec!["hover"] },
            ASTVariant { not: false, pseudo: false, names: vec!["sm"] },
        ],
        elements: vec!["text", "red"],
        arbitrary: Some("200/50"),
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
        head: AstStyle { negative: false, variants: vec![], elements: vec!["w"], arbitrary: None },
        children: vec![
            AstStyle { negative: false, variants: vec![], elements: vec!["full"], arbitrary: None },
            AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["sm"] }],
                elements: vec!["auto"],
                arbitrary: None,
            },
        ],
    };
    assert_eq!(input, output);
    let input = AstGroup::parse("rotate(-3 hover:6 md:(3 hover:-6))").unwrap().1;
    let output = AstGroup {
        head: AstStyle { negative: false, variants: vec![], elements: vec!["w"], arbitrary: None },
        children: vec![
            AstStyle { negative: false, variants: vec![], elements: vec!["full"], arbitrary: None },
            AstStyle {
                negative: false,
                variants: vec![ASTVariant { not: false, pseudo: false, names: vec!["sm"] }],
                elements: vec!["auto"],
                arbitrary: None,
            },
        ],
    };
    assert_eq!(input, output);
}

// #[test]
// fn test_style() {
//     // w-full sm:w-auto text-lg uppercase text-gray-100 bg-purple-800 hover:bg-purple-700 focus:bg-purple-700 focus-visible:ring-4 ring-purple-400 px-6
//     println!("{:#?}", TailwindInstruction::parse("not-hover:sm:text-red-200").unwrap().1);
//     println!("{:#?}", TailwindInstruction::parse_list("w-full sm:w-auto").unwrap().1);
// }
//
// #[test]
// fn test_group() {
//     println!("{:#?}", AstGroup::parse_list("w(full sm:auto)").unwrap().1);
//     println!("{:#?}", AstGroup::parse_list("not-hover:sm:text-red-200").unwrap().1);
// }
//
// #[test]
// fn test_group_expand() {
//     println!("{:#?}", TailwindBuilder::parse_styles("w(full sm:auto)"));
// }
